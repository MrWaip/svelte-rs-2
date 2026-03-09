use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement, VariableDeclarator};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::collections::HashSet;

use svelte_ast::{Attribute, Node, ScriptLanguage};

use crate::builder::{Arg, Builder};
use crate::context::Ctx;

/// Parse and transform the script block.
///
/// Returns `(imports, body)` — imports are extracted separately so they can
/// be hoisted to the top of the generated module.
pub fn gen_script<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let Some(script) = &ctx.component.script else {
        return (vec![], vec![]);
    };

    let script_text = ctx.span_text(script.content_span).to_owned();
    let is_ts = script.language == ScriptLanguage::TypeScript;

    // Compute set of rune names for lookups
    let rune_names: HashSet<String> = ctx.analysis.symbol_by_name
        .iter()
        .filter_map(|(name, sid)| ctx.analysis.runes.contains_key(sid).then(|| name.clone()))
        .collect();

    // First pass: find which rune symbols are mutated (assigned in the script).
    let mut mutated = find_mutated_runes(&script_text, is_ts, &ctx.analysis.symbol_by_name, &ctx.analysis.runes);

    // Also detect bind-implied mutations (bind:value on a rune → rune is mutated)
    let bind_mutated = find_bind_mutations(&ctx.component.fragment, ctx.component, &rune_names);
    mutated.extend(bind_mutated);

    ctx.mutated_runes = mutated;

    // Re-borrow after mutation detection (borrow checker dance).
    let allocator = ctx.b.ast.allocator;
    let script_text = ctx.component.source_text(ctx.component.script.as_ref().unwrap().content_span);

    transform_script_text(allocator, script_text, is_ts, ctx)
}

/// Scan the script source for assignment targets that are rune symbols,
/// returning a set of mutated rune names.
fn find_mutated_runes(
    source: &str,
    is_ts: bool,
    symbol_by_name: &std::collections::HashMap<String, svelte_analyze::SymbolId>,
    runes: &std::collections::HashMap<svelte_analyze::SymbolId, svelte_js::RuneKind>,
) -> HashSet<String> {
    let alloc = Allocator::default();
    let src_type = if is_ts { SourceType::default().with_typescript(true) } else { SourceType::default() };
    let result = OxcParser::new(&alloc, source, src_type).parse();

    let mut mutated = HashSet::new();

    for stmt in &result.program.body {
        collect_mutations_stmt(stmt, symbol_by_name, runes, &mut mutated);
    }

    mutated
}

fn collect_mutations_stmt<'s>(
    stmt: &oxc_ast::ast::Statement<'s>,
    by_name: &std::collections::HashMap<String, svelte_analyze::SymbolId>,
    runes: &std::collections::HashMap<svelte_analyze::SymbolId, svelte_js::RuneKind>,
    out: &mut HashSet<String>,
) {
    use oxc_ast::ast::Statement;
    match stmt {
        Statement::ExpressionStatement(es) => collect_mutations_expr(&es.expression, by_name, runes, out),
        Statement::VariableDeclaration(_) | Statement::FunctionDeclaration(_) => {}
        Statement::BlockStatement(b) => {
            for s in &b.body { collect_mutations_stmt(s, by_name, runes, out); }
        }
        _ => {}
    }
}

fn collect_mutations_expr<'s>(
    expr: &oxc_ast::ast::Expression<'s>,
    by_name: &std::collections::HashMap<String, svelte_analyze::SymbolId>,
    runes: &std::collections::HashMap<svelte_analyze::SymbolId, svelte_js::RuneKind>,
    out: &mut HashSet<String>,
) {
    use oxc_ast::ast::{AssignmentTarget, Expression};
    match expr {
        Expression::AssignmentExpression(assign) => {
            if let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                let name = id.name.as_str();
                if by_name.get(name).is_some_and(|sid| runes.contains_key(sid)) {
                    out.insert(name.to_string());
                }
            }
            collect_mutations_expr(&assign.right, by_name, runes, out);
        }
        Expression::UpdateExpression(upd) => {
            if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
                let name = id.name.as_str();
                if by_name.get(name).is_some_and(|sid| runes.contains_key(sid)) {
                    out.insert(name.to_string());
                }
            }
        }
        Expression::SequenceExpression(seq) => {
            for e in &seq.expressions { collect_mutations_expr(e, by_name, runes, out); }
        }
        _ => {}
    }
}

/// Parse the script source and apply rune transformations, returning (imports, body).
fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    ctx: &Ctx<'a>,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let src_type = if is_ts { SourceType::default().with_typescript(true).with_module(true) } else { SourceType::mjs() };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;

    let mutated = ctx.mutated_runes.clone();
    let rune_names: HashSet<String> = ctx.analysis.symbol_by_name
        .iter()
        .filter_map(|(name, sid)| ctx.analysis.runes.contains_key(sid).then(|| name.clone()))
        .collect();

    let mut transformer = ScriptTransformer { b: &b, mutated_runes: &mutated, rune_names: &rune_names };

    let sem = SemanticBuilder::new().build(&program);
    let (symbols, scopes) = sem.semantic.into_symbol_table_and_scope_tree();
    traverse_mut(&mut transformer, allocator, &mut program, symbols, scopes);

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        if matches!(stmt,
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

struct ScriptTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    mutated_runes: &'b HashSet<String>,
    rune_names: &'b HashSet<String>,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    fn is_rune(&self, name: &str) -> bool {
        self.rune_names.contains(name)
    }

    fn is_mutated(&self, name: &str) -> bool {
        self.mutated_runes.contains(name)
    }

    fn should_proxy(e: &Expression) -> bool {
        if e.is_literal() { return false; }
        if matches!(e,
            Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
        ) { return false; }
        if let Expression::Identifier(id) = e {
            if id.name == "undefined" { return false; }
        }
        true
    }
}

impl<'a> Traverse<'a> for ScriptTransformer<'_, 'a> {
    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a>,
    ) {
        // Get binding name
        let name = match &node.id.kind {
            oxc_ast::ast::BindingPatternKind::BindingIdentifier(id) => id.name.clone(),
            _ => return,
        };

        if !self.is_rune(name.as_str()) {
            return;
        }

        // Has init that is a $state/$derived/etc call?
        let Some(init) = node.init.as_mut() else { return };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            if self.is_mutated(name.as_str()) {
                // Mutated rune → $.state(val)
                call.callee = self.b.rid_expr("$.state");

                // If $state() called with no args, add void 0
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
                // Unmutated rune → inline the value (or void 0 if no args)
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

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        match node {
            Expression::AssignmentExpression(assign) => {
                self.transform_assignment(node, ctx);
            }
            Expression::UpdateExpression(upd) => {
                self.transform_update(node, ctx);
            }
            Expression::Identifier(id) => {
                let name = id.name.as_str().to_string();
                if self.is_rune(&name) && self.is_mutated(&name) {
                    *node = self.b.call_expr("$.get", [Arg::Ident(&name)]);
                }
            }
            _ => {}
        }
    }
}

impl<'a> ScriptTransformer<'_, 'a> {
    fn transform_assignment(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::AssignmentExpression(assign) = node else { return };

        let ident_name = if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
            let name = id.name.as_str();
            if self.is_rune(name) && self.is_mutated(name) {
                Some(name.to_string())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident_name {
            let right = self.b.move_expr(&mut assign.right);
            let call = self.b.call("$.set", [Arg::Ident(&name), Arg::Expr(right)]);
            *node = Expression::CallExpression(self.b.alloc(call));
        }
    }

    fn transform_update(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::UpdateExpression(upd) = node else { return };

        let ident_name = if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
            let name = id.name.as_str();
            if self.is_rune(name) && self.is_mutated(name) {
                Some(name.to_string())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident_name {
            let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
            let is_prefix = upd.prefix;

            // Determine fn name and delta
            let (fn_name, delta): (&str, Option<f64>) = match (is_prefix, is_increment) {
                (true, true) => ("$.update_pre", None),          // ++x
                (true, false) => ("$.update_pre", Some(-1.0)),   // --x
                (false, true) => ("$.update", None),             // x++
                (false, false) => ("$.update", Some(-1.0)),      // x--
            };

            let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
            if let Some(d) = delta {
                args.push(Arg::Num(d));
            }

            *node = self.b.call_expr(fn_name, args);
        }
    }
}

/// Scan template for bind directives that reference rune names.
/// These imply mutation (the setter writes back to the rune).
fn find_bind_mutations(
    fragment: &svelte_ast::Fragment,
    component: &svelte_ast::Component,
    rune_names: &HashSet<String>,
) -> HashSet<String> {
    let mut result = HashSet::new();
    walk_nodes_for_binds(&fragment.nodes, component, rune_names, &mut result);
    result
}

fn walk_nodes_for_binds(
    nodes: &[Node],
    component: &svelte_ast::Component,
    rune_names: &HashSet<String>,
    out: &mut HashSet<String>,
) {
    for node in nodes {
        match node {
            Node::Element(el) => {
                for attr in &el.attributes {
                    if let Attribute::BindDirective(bind) = attr {
                        let name = if bind.shorthand {
                            bind.name.clone()
                        } else if let Some(span) = bind.expression_span {
                            component.source_text(span).trim().to_string()
                        } else {
                            continue;
                        };
                        if rune_names.contains(&name) {
                            out.insert(name);
                        }
                    }
                }
                walk_nodes_for_binds(&el.fragment.nodes, component, rune_names, out);
            }
            Node::IfBlock(b) => {
                walk_nodes_for_binds(&b.consequent.nodes, component, rune_names, out);
                if let Some(alt) = &b.alternate {
                    walk_nodes_for_binds(&alt.nodes, component, rune_names, out);
                }
            }
            Node::EachBlock(b) => {
                walk_nodes_for_binds(&b.body.nodes, component, rune_names, out);
                if let Some(fb) = &b.fallback {
                    walk_nodes_for_binds(&fb.nodes, component, rune_names, out);
                }
            }
            _ => {}
        }
    }
}
