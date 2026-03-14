//! Expression parsing, concatenation building, and emit helpers.

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use rustc_hash::{FxHashMap, FxHashSet};

use svelte_analyze::{ConcatPart, FragmentItem};
use svelte_ast::ConcatPart as AstConcatPart;
use svelte_ast::NodeId;
use svelte_js::{ExpressionKind, RuneKind};
use svelte_span::Span;

use crate::builder::{Arg, AssignLeft, AssignRight, Builder, TemplatePart};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Expression parsing + rune transformation
// ---------------------------------------------------------------------------

pub(crate) fn parse_expr<'a>(ctx: &mut Ctx<'a>, span: Span) -> Expression<'a> {
    let source = ctx.component.source_text(span);
    let mutated = &ctx.analysis.mutated_runes;
    let rune_names = &ctx.analysis.rune_names;
    let snippet_params = &ctx.snippet_param_names;
    let prop_sources = &ctx.prop_sources;
    let prop_non_sources = &ctx.prop_non_sources;
    let each_vars = &ctx.each_vars;

    parse_and_transform(ctx.b.ast.allocator, source, mutated, rune_names, prop_sources, prop_non_sources, snippet_params, each_vars)
}

pub(crate) fn parse_and_transform<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    mutated: &FxHashSet<String>,
    rune_names: &FxHashMap<String, RuneKind>,
    prop_sources: &FxHashSet<String>,
    prop_non_sources: &FxHashMap<String, String>,
    snippet_params: &[String],
    each_vars: &FxHashSet<String>,
) -> Expression<'a> {
    let b = Builder::new(alloc);
    let Ok(expr) = OxcParser::new(alloc, source, SourceType::default()).parse_expression() else {
        debug_assert!(false, "codegen: failed to parse expression: {source}");
        eprintln!("[svelte-rs] warning: failed to parse expression in codegen: {source}");
        return b.str_expr(source);
    };
    let stmt = b.expr_stmt(expr);
    let mut program = b.program(vec![stmt]);

    let mut tr = RuneRefTransformer {
        b: &b,
        mutated,
        rune_names,
        prop_sources,
        prop_non_sources,
        snippet_params,
        each_vars,
    };
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();
    traverse_mut(&mut tr, alloc, &mut program, scoping, ());

    if let Some(Statement::ExpressionStatement(mut es)) = program.body.into_iter().next() {
        b.move_expr(&mut es.expression)
    } else {
        b.str_expr(source)
    }
}

struct RuneRefTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    mutated: &'b FxHashSet<String>,
    rune_names: &'b FxHashMap<String, RuneKind>,
    prop_sources: &'b FxHashSet<String>,
    prop_non_sources: &'b FxHashMap<String, String>,
    snippet_params: &'b [String],
    each_vars: &'b FxHashSet<String>,
}

impl<'a> Traverse<'a, ()> for RuneRefTransformer<'_, 'a> {
    fn enter_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        match node {
            Expression::Identifier(id) => {
                if id.reference_id.get().is_none() {
                    return;
                }
                let name = id.name.as_str().to_string();

                // Snippet params → name() (thunk call)
                if self.snippet_params.iter().any(|p| p == &name) {
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                    return;
                }

                // Props: source → name(), non-source → $$props.name
                if self.prop_sources.contains(&name) {
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                    return;
                }
                if let Some(prop_name) = self.prop_non_sources.get(&name) {
                    *node = self.b.static_member_expr(
                        self.b.rid_expr("$$props"),
                        prop_name,
                    );
                    return;
                }

                // Each-block context variable → $.get(name)
                if self.each_vars.contains(&name) {
                    *node = crate::rune_transform::transform_rune_get(self.b, &name);
                    return;
                }

                // Regular rune: derived always needs $.get(), state only if mutated
                if let Some(&kind) = self.rune_names.get(&name) {
                    let needs_get = self.mutated.contains(&name)
                        || matches!(kind, RuneKind::Derived | RuneKind::DerivedBy);
                    if needs_get {
                        *node = crate::rune_transform::transform_rune_get(self.b, &name);
                    }
                }
            }
            Expression::AssignmentExpression(assign) => {
                let ident_name = if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(
                    id,
                ) = &assign.left
                {
                    let name = id.name.as_str().to_string();
                    (self.rune_names.contains_key(&name) && self.mutated.contains(&name))
                        .then_some(name)
                } else {
                    None
                };

                if let Some(name) = ident_name {
                    let right = self.b.move_expr(&mut assign.right);
                    *node = crate::rune_transform::transform_rune_set(self.b, &name, right, false);
                }
            }
            Expression::UpdateExpression(upd) => {
                let ident_name =
                    if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) =
                        &upd.argument
                    {
                        let name = id.name.as_str().to_string();
                        (self.rune_names.contains_key(&name) && self.mutated.contains(&name))
                            .then_some(name)
                    } else {
                        None
                    };

                if let Some(name) = ident_name {
                    let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
                    *node = crate::rune_transform::transform_rune_update(
                        self.b, &name, upd.prefix, is_increment,
                    );
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Concatenation builders
// ---------------------------------------------------------------------------

pub(crate) fn build_concat<'a>(ctx: &mut Ctx<'a>, item: &FragmentItem) -> Expression<'a> {
    match item {
        FragmentItem::TextConcat { parts, .. } => build_concat_from_parts(ctx, parts),
        _ => ctx.b.str_expr(""),
    }
}

/// Try to resolve an expression tag to a compile-time known value.
fn try_resolve_known(ctx: &Ctx<'_>, nid: NodeId) -> Option<String> {
    let info = ctx.analysis.expressions.get(&nid)?;
    if let ExpressionKind::Identifier(name) = &info.kind {
        ctx.analysis.known_values.get(name.as_str()).cloned()
    } else {
        None
    }
}

pub(crate) fn build_concat_from_parts<'a>(
    ctx: &mut Ctx<'a>,
    parts: &[ConcatPart],
) -> Expression<'a> {
    // Single expr: try constant propagation first
    if parts.len() == 1 {
        if let ConcatPart::Expr(nid) = parts[0] {
            if let Some(val) = try_resolve_known(ctx, nid) {
                return ctx.b.str_expr(&val);
            }
            let span = ctx.expr_span(nid);
            return parse_expr(ctx, span);
        }
    }

    // Multi-part: fold known values into adjacent text
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            ConcatPart::Text(s) => {
                // Merge with previous Str part if possible
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.clone()));
                }
            }
            ConcatPart::Expr(nid) => {
                if let Some(val) = try_resolve_known(ctx, *nid) {
                    // Fold into adjacent text
                    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                        prev.push_str(&val);
                    } else {
                        tpl_parts.push(TemplatePart::Str(val));
                    }
                } else {
                    let span = ctx.expr_span(*nid);
                    let expr = parse_expr(ctx, span);
                    tpl_parts.push(TemplatePart::Expr(expr));
                }
            }
        }
    }

    // If all parts folded into strings, emit a single string literal
    if tpl_parts.len() == 1 {
        if let TemplatePart::Str(s) = &tpl_parts[0] {
            return ctx.b.str_expr(s);
        }
    }

    ctx.b.template_parts_expr(tpl_parts)
}

pub(crate) fn build_attr_concat<'a>(
    ctx: &mut Ctx<'a>,
    parts: &[AstConcatPart],
) -> Expression<'a> {
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            AstConcatPart::Static(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            AstConcatPart::Dynamic(span) => {
                let expr = parse_expr(ctx, *span);
                tpl_parts.push(TemplatePart::Expr(expr));
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

pub(crate) fn static_text_of(item: &FragmentItem) -> String {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts
            .iter()
            .filter_map(|p| {
                if let ConcatPart::Text(s) = p {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect(),
        _ => String::new(),
    }
}

// ---------------------------------------------------------------------------
// Emit helpers
// ---------------------------------------------------------------------------

/// Emit a text update (set_text or nodeValue assignment) depending on dynamism.
pub(crate) fn emit_text_update<'a>(
    ctx: &mut Ctx<'a>,
    item: &FragmentItem,
    node_name: &str,
    body: &mut Vec<Statement<'a>>,
) {
    let is_dyn = item_is_dynamic(item, ctx);
    let expr = build_concat(ctx, item);

    if is_dyn {
        let set = ctx.b.call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(expr)]);
        emit_template_effect(ctx, vec![set], body);
    } else {
        body.push(ctx.b.assign_stmt(
            AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(node_name), "nodeValue")),
            AssignRight::Expr(expr),
        ));
    }
}

pub(crate) fn emit_template_effect<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    let eff = ctx.b.arrow(ctx.b.no_params(), update);
    body.push(ctx.b.call_stmt("$.template_effect", [Arg::Arrow(eff)]));
}

/// Emit `$.next()` or `$.next(N)` for trailing static siblings after the last named var.
pub(crate) fn emit_trailing_next<'a>(
    ctx: &mut Ctx<'a>,
    trailing: usize,
    stmts: &mut Vec<Statement<'a>>,
) {
    if trailing <= 1 {
        return;
    }
    let offset = trailing - 1;
    if offset == 1 {
        stmts.push(ctx.b.call_stmt("$.next", []));
    } else {
        stmts.push(ctx.b.call_stmt("$.next", [Arg::Num(offset as f64)]));
    }
}

// ---------------------------------------------------------------------------
// Analysis helpers
// ---------------------------------------------------------------------------

pub(crate) fn item_is_dynamic(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts_are_dynamic(parts, ctx),
        FragmentItem::Element(id) | FragmentItem::ComponentNode(id) | FragmentItem::IfBlock(id) | FragmentItem::EachBlock(id) | FragmentItem::RenderTag(id) => {
            ctx.analysis.dynamic_nodes.contains(id)
        }
    }
}

pub(crate) fn parts_are_dynamic(parts: &[ConcatPart], ctx: &Ctx<'_>) -> bool {
    parts.iter().any(|p| {
        if let ConcatPart::Expr(id) = p {
            ctx.analysis.dynamic_nodes.contains(id)
        } else {
            false
        }
    })
}
