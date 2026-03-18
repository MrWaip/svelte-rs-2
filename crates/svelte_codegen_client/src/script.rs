use rustc_hash::FxHashSet;

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program, Statement, VariableDeclarator};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use svelte_analyze::{ComponentScoping, PropsAnalysis};
use svelte_ast::ScriptLanguage;
use svelte_js::RuneKind;

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
    if ctx.component.script.is_none() {
        return (vec![], vec![]);
    };

    let allocator = ctx.b.ast.allocator;
    let component_scoping = &ctx.analysis.scoping;
    let props = ctx.analysis.props.as_ref();

    // Take pre-parsed Program from analysis (avoids double-parsing)
    if let Some(program) = ctx.parsed.script_program.take() {
        return transform_program(
            allocator,
            program,
            component_scoping,
            props,
        );
    }

    // Fallback: no pre-parsed program (e.g. tests calling codegen without analysis)
    let script = ctx.component.script.as_ref().unwrap();
    let is_ts = script.language == ScriptLanguage::TypeScript;
    let script_text = ctx.component.source_text(script.content_span);
    transform_script_text(
        allocator,
        script_text,
        is_ts,
        component_scoping,
        props,
        true,
    )
}

/// Transform a standalone JS/TS module (`.svelte.js`/`.svelte.ts`) applying rune rewrites.
/// Unlike component scripts, exports are preserved (not stripped).
pub fn transform_module_script<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    transform_script_text(
        allocator,
        source,
        is_ts,
        component_scoping,
        None,
        false,
    )
}

/// Parse the script source and apply rune transformations, returning (imports, body).
fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
    strip_exports: bool,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let src_type = if is_ts {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;

    // SemanticBuilder populates symbol_id/reference_id on AST nodes,
    // enabling reference resolution during traverse.
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    let props_gen: Option<PropsGenInfo> = props.map(|pa| {
        let root = component_scoping.root_scope_id();
        PropsGenInfo {
            props: pa.props.iter().map(|p| PropGenItem {
                local_name: p.local_name.clone(),
                prop_name: p.prop_name.clone(),
                is_prop_source: p.is_prop_source,
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_mutated: component_scoping.find_binding(root, &p.local_name)
                    .is_some_and(|sym| component_scoping.is_mutated(sym)),
                default_text: p.default_text.clone(),
                is_lazy_default: p.is_lazy_default,
            }).collect(),
        }
    });

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    // Post-traverse: wrap $derived arguments in thunks
    if !transformer.derived_pending.is_empty() {
        wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

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

/// Transform a pre-parsed Program AST (from analysis), applying rune transformations.
fn transform_program<'a>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let b = Builder::new(allocator);

    // Re-run SemanticBuilder to get fresh scoping matching current AST state
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    let props_gen: Option<PropsGenInfo> = props.map(|pa| {
        let root = component_scoping.root_scope_id();
        PropsGenInfo {
            props: pa
                .props
                .iter()
                .map(|p| PropGenItem {
                    local_name: p.local_name.clone(),
                    prop_name: p.prop_name.clone(),
                    is_prop_source: p.is_prop_source,
                    is_bindable: p.is_bindable,
                    is_rest: p.is_rest,
                    is_mutated: component_scoping.find_binding(root, &p.local_name)
                        .is_some_and(|sym| component_scoping.is_mutated(sym)),
                    default_text: p.default_text.clone(),
                    is_lazy_default: p.is_lazy_default,
                })
                .collect(),
        }
    });

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports: true,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    if !transformer.derived_pending.is_empty() {
        wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

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

enum PropKind {
    Source,
    NonSource(String),
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
    /// ComponentScoping — source of truth for rune kind + mutation status.
    component_scoping: &'b ComponentScoping,
    /// OXC scoping from SemanticBuilder — used to resolve references to symbols.
    scoping: Scoping,
    props_gen: Option<PropsGenInfo>,
    /// Names of $derived/$derived.by runes whose init needs post-traverse wrapping.
    derived_pending: FxHashSet<String>,
    /// Whether to strip `export` keywords from declarations. True for component scripts,
    /// false for module compilation where exports must be preserved.
    strip_exports: bool,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    /// Resolve a binding identifier to its rune kind and mutated status.
    /// Only root-scope symbols are considered runes (skips shadowing parameters).
    fn rune_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<(RuneKind, bool)> {
        let sym_id = id.symbol_id.get()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        let root = self.component_scoping.root_scope_id();
        let comp_sym = self.component_scoping.find_binding(root, id.name.as_str())?;
        let kind = self.component_scoping.rune_kind(comp_sym)?;
        Some((kind, self.component_scoping.is_mutated(comp_sym)))
    }

    /// Resolve a reference identifier to its rune kind and mutated status.
    fn rune_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<(RuneKind, bool)> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        let root = self.component_scoping.root_scope_id();
        let comp_sym = self.component_scoping.find_binding(root, id.name.as_str())?;
        let kind = self.component_scoping.rune_kind(comp_sym)?;
        Some((kind, self.component_scoping.is_mutated(comp_sym)))
    }

    /// Resolve a reference identifier to its prop kind (source or non-source).
    fn prop_kind_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<PropKind> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        let root = self.component_scoping.root_scope_id();
        let comp_sym = self.component_scoping.find_binding(root, id.name.as_str())?;
        if self.component_scoping.is_prop_source(comp_sym) {
            Some(PropKind::Source)
        } else if let Some(prop_name) = self.component_scoping.prop_non_source_name(comp_sym) {
            Some(PropKind::NonSource(prop_name.to_string()))
        } else {
            None
        }
    }

    /// Check if an identifier name is a `$store` reference (e.g. `$count` where `count` is a store).
    fn is_store_ref(&self, name: &str) -> bool {
        if name.starts_with('$') && name.len() > 1 {
            let base = &name[1..];
            let root = self.component_scoping.root_scope_id();
            return self.component_scoping.find_binding(root, base)
                .is_some_and(|sym| self.component_scoping.is_store(sym));
        }
        false
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
                // Wrap $bindable() defaults in $.proxy() when needed
                let default_expr = if prop.is_bindable && Self::should_proxy(&default_expr) {
                    self.b.call_expr("$.proxy", [Arg::Expr(default_expr)])
                } else {
                    default_expr
                };
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

/// Post-traverse: wrap `$.derived(expr)` → `$.derived(() => expr)` for $derived runes.
fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut oxc_ast::ast::Program<'a>,
    names: &FxHashSet<String>,
) {
    use oxc_ast::ast::Statement;
    for stmt in program.body.iter_mut() {
        if let Statement::VariableDeclaration(decl) = stmt {
            for declarator in decl.declarations.iter_mut() {
                let name = match &declarator.id {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => id.name.as_str(),
                    _ => continue,
                };
                if !names.contains(name) {
                    continue;
                }
                if let Some(Expression::CallExpression(call)) = &mut declarator.init {
                    if !call.arguments.is_empty() {
                        let mut dummy = oxc_ast::ast::Argument::from(b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg_expr = dummy.into_expression();
                        let thunk = b.thunk(arg_expr);
                        call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                    }
                }
            }
        }
    }
}

fn parse_expression<'a>(b: &Builder<'a>, text: &str) -> Expression<'a> {
    let alloc = b.ast.allocator;
    // Allocate text in the arena so it lives long enough for OXC parsing
    let arena_text: &'a str = alloc.alloc_str(text);
    match OxcParser::new(alloc, arena_text, SourceType::default()).parse_expression() {
        Ok(expr) => expr,
        Err(_) => {
            debug_assert!(false, "codegen: failed to parse expression: {text}");
            eprintln!("[svelte-rs] warning: failed to parse expression in script codegen: {text}");
            b.str_expr(text)
        }
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
        // (only for component scripts; module compilation preserves exports)
        if self.strip_exports {
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
                self.rune_for_binding(id)
            }
            _ => return,
        };

        let Some((kind, mutated)) = rune_info else {
            return;
        };

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            match kind {
                RuneKind::Derived => {
                    // $derived(expr) → $.derived(() => expr)
                    // Just rewrite callee here; thunk wrapping happens post-traverse
                    // to avoid OXC scope_id issues with new arrow nodes.
                    call.callee = self.b.rid_expr("$.derived");
                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(bid) = &node.id {
                        self.derived_pending.insert(bid.name.as_str().to_string());
                    }
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::DerivedBy => {
                    // $derived.by(fn) → $.derived(fn)
                    call.callee = self.b.rid_expr("$.derived");
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::State | RuneKind::StateRaw => {
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
                        let value = if kind == RuneKind::State && Self::should_proxy(&value) {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        node.init = Some(value);
                    }
                }
                _ => {
                    // Other rune kinds — put back the call unchanged
                    node.init = Some(Expression::CallExpression(call));
                }
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
            Expression::CallExpression(call) => {
                let new_callee = match &call.callee {
                    Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                        Some("$.user_effect")
                    }
                    Expression::StaticMemberExpression(member) => {
                        if let Expression::Identifier(obj) = &member.object {
                            match (obj.name.as_str(), member.property.name.as_str()) {
                                ("$effect", "pre") => Some("$.user_pre_effect"),
                                ("$effect", "root") => Some("$.effect_root"),
                                ("$state", "snapshot") => Some("$.snapshot"),
                                ("$effect", "tracking") => Some("$.effect_tracking"),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(callee_name) = new_callee {
                    let Expression::CallExpression(call) = node else { unreachable!() };
                    call.callee = self.b.rid_expr(callee_name);
                }
            }
            Expression::Identifier(id) => {
                // Check props first
                if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                    match prop_kind {
                        PropKind::Source => {
                            let name = id.name.as_str().to_string();
                            *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                        }
                        PropKind::NonSource(prop_name) => {
                            *node = self.b.static_member_expr(
                                self.b.rid_expr("$$props"),
                                &prop_name,
                            );
                        }
                    }
                    return;
                }
                // Store subscription read: $count → $count()
                // Only transform original source identifiers (with reference_id),
                // not synthetic ones we create during transformation.
                let id_name = id.name.as_str();
                if id.reference_id.get().is_some() && self.is_store_ref(id_name) {
                    let name = id_name.to_string();
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                    return;
                }
                // Regular rune check
                let Some((kind, mutated)) = self.rune_for_ref(id) else {
                    return;
                };
                let needs_get = mutated
                    || kind.is_derived();
                if needs_get {
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
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }
            }
            // Store subscription assignment: $count = val → $.store_set(count, val)
            // Compound: $count += val → $.store_set(count, $count() + val)
            let id_name = id.name.as_str();
            if self.is_store_ref(id_name) {
                let base_name: &str = self.b.alloc_str(&id_name[1..]);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let right = self.b.move_expr(&mut assign.right);

                let value = if assign.operator.is_assign() {
                    right
                } else {
                    // Read current value via thunk call: $count()
                    let current = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                    if let Some(bin_op) = assign.operator.to_binary_operator() {
                        self.b.ast.expression_binary(oxc_span::SPAN, current, bin_op, right)
                    } else if let Some(log_op) = assign.operator.to_logical_operator() {
                        self.b.ast.expression_logical(oxc_span::SPAN, current, log_op, right)
                    } else {
                        unreachable!("all compound assignment operators are either binary or logical")
                    }
                };

                *node = self.b.call_expr("$.store_set", [
                    Arg::Ident(base_name),
                    Arg::Expr(value),
                ]);
                return;
            }
            if let Some((kind, mutated)) = self.rune_for_ref(id) {
                if mutated {
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

                    let needs_proxy = kind != RuneKind::StateRaw && Self::should_proxy(&value);
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
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
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
            // Store subscription update: $count++ → $.update_store(count, $count())
            // ++$count → $.update_pre_store(count, $count())
            // $count-- → $.update_store(count, $count(), -1)
            let id_name = id.name.as_str();
            if self.is_store_ref(id_name) {
                let base_name: &str = self.b.alloc_str(&id_name[1..]);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let fn_name = if upd.prefix { "$.update_pre_store" } else { "$.update_store" };
                let thunk_call = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                let mut args: Vec<Arg<'a, '_>> = vec![
                    Arg::Ident(base_name),
                    Arg::Expr(thunk_call),
                ];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
            if let Some((_, mutated)) = self.rune_for_ref(id) {
                if mutated {
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
