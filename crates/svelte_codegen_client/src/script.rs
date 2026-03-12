use std::collections::{HashMap, HashSet};

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement, VariableDeclarator};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use svelte_analyze::PropsAnalysis;
use svelte_ast::ScriptLanguage;

use crate::builder::{Arg, Builder};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Props flag constants (must match svelte/src/constants.js)
// ---------------------------------------------------------------------------

const PROPS_IS_IMMUTABLE: u32 = 1;
const PROPS_IS_RUNES: u32 = 1 << 1;
const PROPS_IS_UPDATED: u32 = 1 << 2;
const PROPS_IS_BINDABLE: u32 = 1 << 3;
const PROPS_IS_LAZY_INITIAL: u32 = 1 << 4;

/// Parse and transform the script block.
///
/// Returns `(imports, body)` — imports are extracted separately so they can
/// be hoisted to the top of the generated module.
pub fn gen_script<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let Some(script) = &ctx.component.script else {
        return (vec![], vec![]);
    };

    let is_ts = script.language == ScriptLanguage::TypeScript;
    let allocator = ctx.b.ast.allocator;
    let script_text = ctx.component.source_text(script.content_span);

    let rune_names = &ctx.analysis.rune_names;
    let mutated_runes = &ctx.analysis.mutated_runes;
    let props = ctx.analysis.props.as_ref();

    transform_script_text(
        allocator,
        script_text,
        is_ts,
        rune_names,
        mutated_runes,
        props,
        &ctx.prop_sources,
        &ctx.prop_non_sources,
    )
}

/// Parse the script source and apply rune transformations, returning (imports, body).
fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    rune_names: &HashSet<String>,
    mutated_runes: &HashSet<String>,
    props: Option<&PropsAnalysis>,
    prop_sources: &HashSet<String>,
    prop_non_sources: &HashMap<String, String>,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let src_type = if is_ts {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;

    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    // Build rune_table and prop_table from OXC symbols
    let mut rune_table: HashMap<oxc_semantic::SymbolId, RuneInfo> = HashMap::new();
    let mut prop_table: HashMap<oxc_semantic::SymbolId, PropSymInfo> = HashMap::new();

    let root_scope = scoping.root_scope_id();
    for sym_id in scoping.symbol_ids() {
        let name = scoping.symbol_name(sym_id);
        if prop_sources.contains(name) {
            prop_table.insert(sym_id, PropSymInfo { kind: PropKind::Source });
        } else if prop_non_sources.contains_key(name) {
            prop_table.insert(sym_id, PropSymInfo {
                kind: PropKind::NonSource(name.to_string()),
            });
        } else if rune_names.contains(name) {
            // Only treat symbols from root scope as runes — skip parameters
            // that shadow a rune variable
            if scoping.symbol_scope_id(sym_id) == root_scope {
                let mutated = mutated_runes.contains(name);
                rune_table.insert(sym_id, RuneInfo { mutated });
            }
        }
    }

    let props_gen: Option<PropsGenInfo> = props.map(|pa| {
        PropsGenInfo {
            props: pa.props.iter().map(|p| PropGenItem {
                local_name: p.local_name.clone(),
                prop_name: p.prop_name.clone(),
                is_prop_source: p.is_prop_source,
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_mutated: mutated_runes.contains(&p.local_name),
                default_text: p.default_text.clone(),
                is_lazy_default: p.is_lazy_default,
            }).collect(),
        }
    });

    let mut transformer = ScriptTransformer {
        b: &b,
        rune_table,
        prop_table,
        scoping,
        props_gen,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        if matches!(
            stmt,
            Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSEnumDeclaration(_)
        ) {
            continue;
        }
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            imports.push(stmt);
        } else {
            body.push(stmt);
        }
    }

    (imports, body)
}

struct RuneInfo {
    mutated: bool,
}

#[derive(Clone)]
enum PropKind {
    Source,
    NonSource(String),
}

struct PropSymInfo {
    kind: PropKind,
}

struct PropsGenInfo {
    props: Vec<PropGenItem>,
}

struct PropGenItem {
    local_name: String,
    prop_name: String,
    is_prop_source: bool,
    is_bindable: bool,
    is_rest: bool,
    is_mutated: bool,
    default_text: Option<String>,
    is_lazy_default: bool,
}

struct ScriptTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    rune_table: HashMap<oxc_semantic::SymbolId, RuneInfo>,
    prop_table: HashMap<oxc_semantic::SymbolId, PropSymInfo>,
    scoping: Scoping,
    props_gen: Option<PropsGenInfo>,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    fn rune_info_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<&RuneInfo> {
        let sym_id = id.symbol_id.get()?;
        self.rune_table.get(&sym_id)
    }

    fn rune_info_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<&RuneInfo> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        self.rune_table.get(&sym_id)
    }

    fn prop_info_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<&PropSymInfo> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        self.prop_table.get(&sym_id)
    }

    fn should_proxy(e: &Expression) -> bool {
        if e.is_literal() {
            return false;
        }
        if matches!(
            e,
            Expression::TemplateLiteral(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::UnaryExpression(_)
                | Expression::BinaryExpression(_)
        ) {
            return false;
        }
        if let Expression::Identifier(id) = e {
            if id.name == "undefined" {
                return false;
            }
        }
        true
    }

    fn is_props_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let oxc_ast::ast::BindingPattern::ObjectPattern(_) = &d.id {
                if let Some(init) = &d.init {
                    if let Expression::CallExpression(call) = init {
                        if let Expression::Identifier(ident) = &call.callee {
                            return ident.name.as_str() == "$props";
                        }
                    }
                }
            }
            false
        })
    }

    fn gen_props_statements(&self) -> Vec<Statement<'a>> {
        let Some(props_gen) = &self.props_gen else {
            return vec![];
        };

        let mut declarators: Vec<(&str, Expression<'a>)> = Vec::new();
        let mut seen_names: Vec<String> = vec![
            "$$slots".to_string(),
            "$$events".to_string(),
            "$$legacy".to_string(),
        ];

        for prop in &props_gen.props {
            seen_names.push(prop.prop_name.clone());

            if prop.is_rest {
                let excluded: Vec<Arg<'a, '_>> = seen_names.iter()
                    .filter(|n| *n != &prop.local_name)
                    .map(|n| Arg::Str(n.clone()))
                    .collect();
                let arr_expr = self.b.array_from_args(excluded);
                let init = self.b.call_expr("$.rest_props", [
                    Arg::Ident("$$props"),
                    Arg::Expr(arr_expr),
                ]);
                declarators.push((self.b.alloc_str(&prop.local_name), init));
                continue;
            }

            if !prop.is_prop_source {
                continue;
            }

            let mut flags: u32 = PROPS_IS_IMMUTABLE | PROPS_IS_RUNES;
            if prop.is_bindable {
                flags |= PROPS_IS_BINDABLE;
            }
            if prop.is_mutated {
                flags |= PROPS_IS_UPDATED;
            }

            let mut args: Vec<Arg<'a, '_>> = vec![
                Arg::Ident("$$props"),
                Arg::Str(prop.prop_name.clone()),
            ];

            if let Some(default_text) = &prop.default_text {
                if prop.is_lazy_default {
                    flags |= PROPS_IS_LAZY_INITIAL;
                }

                args.push(Arg::Num(flags as f64));

                // Parse default expression
                let default_expr = parse_expression(self.b, default_text);
                if !prop.is_lazy_default {
                    args.push(Arg::Expr(default_expr));
                } else {
                    args.push(Arg::Expr(wrap_lazy(self.b, default_expr)));
                }
            } else {
                if flags != 0 {
                    args.push(Arg::Num(flags as f64));
                }
            }

            let name: &'a str = self.b.alloc_str(&prop.local_name);
            declarators.push((name, self.b.call_expr("$.prop", args)));
        }

        if declarators.is_empty() {
            return vec![];
        }

        vec![self.b.let_multi_stmt(declarators)]
    }
}

fn parse_expression<'a>(b: &Builder<'a>, text: &str) -> Expression<'a> {
    let alloc = b.ast.allocator;
    // Allocate text in the arena so it lives long enough for OXC parsing
    let arena_text: &'a str = alloc.alloc_str(text);
    match OxcParser::new(alloc, arena_text, SourceType::default()).parse_expression() {
        Ok(expr) => expr,
        Err(_) => b.str_expr(text),
    }
}

/// Wrap a non-simple default expression for lazy evaluation.
fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    // Zero-arg call foo() → use callee directly (already lazy)
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty() {
            if let Expression::Identifier(_) = &call.callee {
                return b.clone_expr(&call.callee);
            }
        }
    }
    // Otherwise wrap: () => expr
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}

impl<'a> Traverse<'a, ()> for ScriptTransformer<'_, 'a> {
    fn exit_statements(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Strip `export` keyword: ExportNamedDeclaration → inner declaration
        let mut i = 0;
        while i < stmts.len() {
            if let Statement::ExportNamedDeclaration(_) = &stmts[i] {
                let stmt = stmts.remove(i);
                if let Statement::ExportNamedDeclaration(export) = stmt {
                    if let Some(decl) = export.unbox().declaration {
                        stmts.insert(i, Statement::from(decl));
                        i += 1;
                    }
                    // else: `export { x }` form — just remove
                }
            } else {
                i += 1;
            }
        }

        // Replace $props() destructuring
        if self.props_gen.is_none() {
            return;
        }

        let mut idx = None;
        for (j, stmt) in stmts.iter().enumerate() {
            if let Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_declaration(decl) {
                    idx = Some(j);
                    break;
                }
            }
        }

        let Some(j) = idx else { return };

        let replacement = self.gen_props_statements();
        stmts.remove(j);
        for (k, stmt) in replacement.into_iter().enumerate() {
            stmts.insert(j + k, stmt);
        }
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let rune_info = match &node.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                self.rune_info_for_binding(id)
            }
            _ => return,
        };

        let Some(info) = rune_info else {
            return;
        };
        let mutated = info.mutated;

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            if mutated {
                call.callee = self.b.rid_expr("$.state");

                if call.arguments.is_empty() {
                    let void_zero = self.b.ast.expression_unary(
                        oxc_span::SPAN,
                        oxc_ast::ast::UnaryOperator::Void,
                        self.b.num_expr(0.0),
                    );
                    call.arguments.push(void_zero.into());
                }

                node.init = Some(Expression::CallExpression(call));
            } else {
                let value = if call.arguments.is_empty() {
                    self.b.ast.expression_unary(
                        oxc_span::SPAN,
                        oxc_ast::ast::UnaryOperator::Void,
                        self.b.num_expr(0.0),
                    )
                } else {
                    let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                    std::mem::swap(&mut call.arguments[0], &mut dummy);
                    dummy.into_expression()
                };
                let value = if Self::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                node.init = Some(value);
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        match node {
            Expression::AssignmentExpression(_) => {
                self.transform_assignment(node, ctx);
            }
            Expression::UpdateExpression(_) => {
                self.transform_update(node, ctx);
            }
            Expression::Identifier(id) => {
                // Check props first
                if let Some(prop_info) = self.prop_info_for_ref(id) {
                    match &prop_info.kind {
                        PropKind::Source => {
                            let name = id.name.as_str().to_string();
                            *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                        }
                        PropKind::NonSource(prop_name) => {
                            let prop_name = prop_name.clone();
                            *node = self.b.static_member_expr(
                                self.b.rid_expr("$$props"),
                                &prop_name,
                            );
                        }
                    }
                    return;
                }
                // Regular rune check
                let Some(info) = self.rune_info_for_ref(id) else {
                    return;
                };
                if info.mutated {
                    let name = id.name.as_str().to_string();
                    *node = crate::rune_transform::transform_rune_get(self.b, &name);
                }
            }
            _ => {}
        }
    }
}

impl<'a> ScriptTransformer<'_, 'a> {
    fn transform_assignment(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };

        if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
            if let Some(prop_info) = self.prop_info_for_ref(id) {
                if matches!(prop_info.kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }
            }
            if let Some(info) = self.rune_info_for_ref(id) {
                if info.mutated {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);

                    // Expand compound assignments: value += x → $.set(value, $.get(value) + x)
                    let value = if assign.operator.is_assign() {
                        right
                    } else {
                        let left_get = crate::rune_transform::transform_rune_get(self.b, &name);
                        if let Some(bin_op) = assign.operator.to_binary_operator() {
                            self.b.ast.expression_binary(oxc_span::SPAN, left_get, bin_op, right)
                        } else if let Some(log_op) = assign.operator.to_logical_operator() {
                            self.b.ast.expression_logical(oxc_span::SPAN, left_get, log_op, right)
                        } else {
                            unreachable!("all compound assignment operators are either binary or logical")
                        }
                    };

                    let needs_proxy = Self::should_proxy(&value);
                    *node = crate::rune_transform::transform_rune_set(self.b, &name, value, needs_proxy);
                    return;
                }
            }
        }
    }

    fn transform_update(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
            if let Some(prop_info) = self.prop_info_for_ref(id) {
                if matches!(prop_info.kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let fn_name = if upd.prefix { "$.update_pre_prop" } else { "$.update_prop" };
                    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
                    if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                        args.push(Arg::Num(-1.0));
                    }
                    *node = self.b.call_expr(fn_name, args);
                    return;
                }
            }
            if let Some(info) = self.rune_info_for_ref(id) {
                if info.mutated {
                    let name = id.name.as_str().to_string();
                    let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
                    *node = crate::rune_transform::transform_rune_update(
                        self.b, &name, upd.prefix, is_increment,
                    );
                    return;
                }
            }
        }
    }
}
