//! Template and DOM traversal code generation.

pub(crate) mod async_plan;
pub(crate) mod attributes;
pub(crate) mod await_block;
pub(crate) mod bind;

pub(crate) mod component;
pub(crate) mod const_tag;
pub(crate) mod debug_tag;
pub(crate) mod each_block;
pub(crate) mod element;
pub(crate) mod events;
pub(crate) mod expression;
pub(crate) mod html;
pub(crate) mod html_tag;
pub(crate) mod if_block;
pub(crate) mod key_block;
pub(crate) mod render_tag;
pub(crate) mod snippet;
pub(crate) mod svelte_body;
pub(crate) mod svelte_boundary;
pub(crate) mod svelte_document;
pub(crate) mod svelte_element;
pub(crate) mod svelte_head;
pub(crate) mod svelte_window;
pub(crate) mod title_element;
pub(crate) mod traverse;

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ContentStrategy, FragmentItem, FragmentKey};
use svelte_ast::{Node, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use element::process_element;

use crate::script::compute_line_col;

/// Wrap a block expression with `$.add_svelte_meta` in dev mode.
///
/// In production: returns `ctx.b.expr_stmt(expression)`.
/// In dev: returns `$.add_svelte_meta(() => expr, type, ComponentName, line, col)`.
pub(crate) fn add_svelte_meta<'a>(
    ctx: &Ctx<'a>,
    expression: Expression<'a>,
    span_start: u32,
    block_type: &str,
) -> Statement<'a> {
    if !ctx.state.dev {
        return ctx.b.expr_stmt(expression);
    }

    let (line, col) = compute_line_col(ctx.state.source, span_start);

    let thunk = ctx
        .b
        .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(expression)]);
    ctx.b.call_stmt(
        "$.add_svelte_meta",
        [
            Arg::Expr(thunk),
            Arg::StrRef(block_type),
            Arg::Ident(ctx.state.name),
            Arg::Num(line as f64),
            Arg::Num(col as f64),
        ],
    )
}

/// Return the appropriate `$.from_*` function for the component's namespace.
fn from_template_fn(ctx: &Ctx) -> &'static str {
    use svelte_ast::Namespace;
    match ctx
        .query
        .component
        .options
        .as_ref()
        .and_then(|o| o.namespace.as_ref())
    {
        Some(Namespace::Svg) => "$.from_svg",
        Some(Namespace::Mathml) => "$.from_mathml",
        _ => "$.from_html",
    }
}

/// Return the `$.from_*` function based on the element's tag name.
/// An inline `<svg>` in an HTML component needs `$.from_svg`, not `$.from_html`.
fn from_template_fn_for_element(ctx: &Ctx, el_name: &str) -> &'static str {
    if svelte_ast::is_svg(el_name) {
        "$.from_svg"
    } else if svelte_ast::is_mathml(el_name) {
        "$.from_mathml"
    } else {
        from_template_fn(ctx)
    }
}

/// Infer the `$.from_*` function from the first element in a fragment's items.
fn from_template_fn_for_items(ctx: &Ctx, items: &[FragmentItem]) -> &'static str {
    for item in items {
        if let FragmentItem::Element(el_id) = item {
            let el = ctx.element(*el_id);
            return from_template_fn_for_element(ctx, &el.name);
        }
    }
    from_template_fn(ctx)
}
use await_block::gen_await_block;
use component::gen_component;
use const_tag::gen_const_tags;
use debug_tag::emit_debug_tags;
use each_block::gen_each_block;
use expression::{emit_text_update, emit_trailing_next};
use html::{element_html, fragment_html};
use html_tag::gen_html_tag;
use if_block::gen_if_block;
use key_block::gen_key_block;
use render_tag::gen_render_tag;
use svelte_boundary::gen_svelte_boundary;
use svelte_element::gen_svelte_element;
use title_element::emit_title_elements;
use traverse::traverse_items;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Returns `(hoisted, body, snippet_stmts, hoistable_snippets)` for the root fragment.
/// `snippet_stmts` are instance-level snippets that go inside the function body.
/// `hoistable_snippets` are module-level snippet declarations.
pub fn gen_root_fragment<'a>(
    ctx: &mut Ctx<'a>,
) -> (
    Vec<Statement<'a>>,
    Vec<Statement<'a>>,
    Vec<Statement<'a>>,
    Vec<Statement<'a>>,
) {
    let key = FragmentKey::Root;
    let ct = ctx.content_type(&key);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    // Collect IDs for special elements, snippets — process in source order
    // to match Svelte reference ident numbering.
    let mut svelte_head_ids = Vec::new();
    let mut svelte_window_ids = Vec::new();
    let mut svelte_document_ids = Vec::new();
    let mut svelte_body_ids = Vec::new();
    let mut instance_snippet_ids = Vec::new();
    let mut hoistable_snippet_ids = Vec::new();
    for &id in &ctx.query.component.fragment.nodes {
        let node = ctx.query.component.store.get(id);
        match node {
            svelte_ast::Node::SvelteHead(h) => svelte_head_ids.push(h.id),
            svelte_ast::Node::SvelteWindow(w) => svelte_window_ids.push(w.id),
            svelte_ast::Node::SvelteDocument(d) => svelte_document_ids.push(d.id),
            svelte_ast::Node::SvelteBody(b) => svelte_body_ids.push(b.id),
            svelte_ast::Node::SnippetBlock(block) => {
                if ctx.is_snippet_hoistable(block.id) {
                    hoistable_snippet_ids.push(block.id);
                } else {
                    instance_snippet_ids.push(block.id);
                }
            }
            _ => {}
        }
    }

    // Svelte reference processes hoisted nodes in source order: head first,
    // then snippets. This determines template ident numbering (root_1, root_2, ...).
    let mut head_stmts = Vec::new();
    for id in &svelte_head_ids {
        svelte_head::gen_svelte_head(ctx, *id, &mut head_stmts);
    }

    // Generate snippet bodies: instance first, then hoistable
    let mut snippet_stmts: Vec<Statement<'a>> = Vec::new();
    for id in instance_snippet_ids {
        snippet_stmts.push(snippet::gen_snippet_block(ctx, id, vec![]));
    }
    let mut hoistable_snippets: Vec<Statement<'a>> = Vec::new();
    for id in hoistable_snippet_ids {
        hoistable_snippets.push(snippet::gen_snippet_block(ctx, id, vec![]));
    }

    let mut hoisted = Vec::with_capacity(4);
    let mut body = Vec::with_capacity(8);

    let async_const_run = gen_const_tags(ctx, key.clone(), &mut body);
    emit_debug_tags(ctx, key.clone(), &mut body);
    if let Some(run_stmt) = async_const_run {
        body.push(run_stmt);
    }

    // Template init first (Svelte reference unshifts var decl to init[0])
    let pre_len = body.len();
    emit_content_strategy(ctx, key, &ct, &tpl_name, true, &mut hoisted, &mut body);

    // Svelte reference order in init[]: template var (unshifted to [0]) →
    // head → window/document/body events → child processing → append.

    // Special element events/bindings
    let mut special_body: Vec<Statement<'a>> = Vec::new();
    for id in svelte_window_ids {
        svelte_window::gen_svelte_window(ctx, id, &mut special_body);
    }
    for id in svelte_document_ids {
        svelte_document::gen_svelte_document(ctx, id, &mut special_body);
    }
    for id in svelte_body_ids {
        svelte_body::gen_svelte_body(ctx, id, &mut special_body);
    }

    // Insert head + special elements right after root var init
    let combined: Vec<_> = head_stmts.into_iter().chain(special_body).collect();
    if !combined.is_empty() {
        if body.len() > pre_len + 1 {
            let insert_pos = pre_len + 1;
            let tail: Vec<_> = body.drain(insert_pos..).collect();
            body.extend(combined);
            body.extend(tail);
        } else {
            body.extend(combined);
        }
    }

    (hoisted, body, snippet_stmts, hoistable_snippets)
}

// ---------------------------------------------------------------------------
// Fragment codegen (used for if/each/nested fragments)
// ---------------------------------------------------------------------------

/// Generate statements for a fragment, destined for `($$anchor) => { ... }`.
pub(crate) fn gen_fragment<'a>(ctx: &mut Ctx<'a>, key: FragmentKey) -> Vec<Statement<'a>> {
    let ct = ctx.content_type(&key);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    let mut body: Vec<Statement<'a>> = Vec::with_capacity(8);

    let async_const_run = gen_const_tags(ctx, key.clone(), &mut body);
    emit_debug_tags(ctx, key.clone(), &mut body);
    if let Some(run_stmt) = async_const_run {
        body.push(run_stmt);
    }

    let mut sub_hoisted = Vec::with_capacity(2);
    emit_content_strategy(ctx, key.clone(), &ct, &tpl_name, false, &mut sub_hoisted, &mut body);
    ctx.state.module_hoisted.extend(sub_hoisted);

    // Title elements emit after DOM init but before $.append()
    let has_append = matches!(
        ct,
        ContentStrategy::SingleElement(_)
            | ContentStrategy::DynamicText
            | ContentStrategy::Mixed { .. }
            | ContentStrategy::SingleBlock(_)
    );
    if has_append && !body.is_empty() {
        let append = body.pop().unwrap();
        emit_title_elements(ctx, key, &mut body);
        body.push(append);
    } else {
        emit_title_elements(ctx, key, &mut body);
    }

    body
}

// ---------------------------------------------------------------------------
// Shared content strategy dispatch
// ---------------------------------------------------------------------------

/// Emit body statements for a given ContentStrategy.
///
/// `is_root` controls whether `$.next()` is emitted for Static/DynamicText
/// and determines the hoisting order for templates:
/// - Root (is_root=true): template is pushed to `hoisted` BEFORE children
///   (top-down order; the caller places hoisted after module_hoisted in output).
/// - Non-root (is_root=false): template is pushed to `hoisted` AFTER children
///   (bottom-up order; the caller extends module_hoisted with the local vec).
fn emit_content_strategy<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    ct: &ContentStrategy,
    tpl_name: &str,
    is_root: bool,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    match ct {
        ContentStrategy::Empty => {}
        ContentStrategy::Static(ref text) => {
            emit_static_text(ctx, text, is_root, body);
        }
        ContentStrategy::DynamicText => {
            emit_dynamic_text(ctx, key, is_root, body);
        }
        ContentStrategy::SingleElement(el_id) => {
            emit_single_element(ctx, *el_id, tpl_name, is_root, hoisted, body);
        }
        ContentStrategy::SingleBlock(ref item) => {
            emit_single_block(ctx, item, is_root, body);
        }
        ContentStrategy::Mixed { .. } => {
            emit_mixed(ctx, key, tpl_name, is_root, hoisted, body);
        }
    }
}

// ---------------------------------------------------------------------------
// Strategy implementations
// ---------------------------------------------------------------------------

fn emit_static_text<'a>(
    ctx: &mut Ctx<'a>,
    text: &str,
    is_root: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let name = ctx.gen_ident("text");
    if is_root {
        body.push(ctx.b.call_stmt("$.next", []));
    }
    let call = if text.is_empty() {
        ctx.b.call_expr("$.text", [])
    } else {
        ctx.b.call_expr("$.text", [Arg::StrRef(text)])
    };
    body.push(ctx.b.var_stmt(&name, call));
    body.push(
        ctx.b
            .call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]),
    );
}

fn emit_dynamic_text<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    is_root: bool,
    body: &mut Vec<Statement<'a>>,
) {
    // Clone needed: emit_text_update borrows ctx mutably
    let item = ctx.lowered_fragment(&key).items[0].clone();
    // Reference compiler allocates a "fragment" ident before checking use_space_template,
    // then discards it. We must match to keep counters in sync.
    if !is_root {
        ctx.gen_ident("fragment");
    }
    let name = ctx.gen_ident("text");
    if is_root {
        body.push(ctx.b.call_stmt("$.next", []));
    }
    body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));
    emit_text_update(ctx, &item, &name, body);
    body.push(
        ctx.b
            .call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&name)]),
    );
}

fn emit_single_element<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    tpl_name: &str,
    is_root: bool,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    let el = ctx.element(el_id);
    let (html, import_node) = element_html(ctx, el);
    let from_fn = from_template_fn_for_element(ctx, &el.name);
    let mut from_html = if import_node {
        ctx.b.call_expr(
            from_fn,
            [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(2.0)],
        )
    } else {
        ctx.b
            .call_expr(from_fn, [Arg::Expr(ctx.b.template_str_expr(&html))])
    };
    if ctx.state.dev {
        let locs = build_element_locations(ctx, el_id);
        from_html = wrap_add_locations(ctx, from_html, locs);
    }
    let tpl_stmt = ctx.b.var_stmt(tpl_name, from_html);

    let el_name_str = ctx.element(el_id).name.clone();
    let el_name = ctx.gen_ident(&el_name_str);

    // Pre-computed blocker indices for this element's fragment
    let el_key = svelte_analyze::FragmentKey::Element(el_id);
    let el_blockers = ctx.fragment_blockers(&el_key).to_vec();

    if is_root {
        // Root: template BEFORE children (top-down)
        hoisted.push(tpl_stmt);
        body.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(tpl_name, [])));
        let mut init = Vec::with_capacity(8);
        let mut update = Vec::with_capacity(4);
        let mut after_update = Vec::with_capacity(4);
        let mut memo_attrs = Vec::new();
        process_element(
            ctx,
            el_id,
            &el_name,
            &mut init,
            &mut update,
            hoisted,
            &mut after_update,
            &mut memo_attrs,
        );
        body.extend(init);
        let local_blockers = expression::build_fragment_local_blockers(ctx, &el_key);
        expression::emit_template_effect_with_memo(
            ctx,
            update,
            memo_attrs,
            el_blockers,
            local_blockers,
            body,
        );
        body.extend(after_update);
    } else {
        // Non-root: template AFTER children (bottom-up)
        let mut init = Vec::with_capacity(8);
        let mut update = Vec::with_capacity(4);
        let mut after_update = Vec::with_capacity(4);
        let mut memo_attrs = Vec::new();
        process_element(
            ctx,
            el_id,
            &el_name,
            &mut init,
            &mut update,
            hoisted,
            &mut after_update,
            &mut memo_attrs,
        );
        hoisted.push(tpl_stmt);
        body.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(tpl_name, [])));
        body.extend(init);
        let local_blockers = expression::build_fragment_local_blockers(ctx, &el_key);
        expression::emit_template_effect_with_memo(
            ctx,
            update,
            memo_attrs,
            el_blockers,
            local_blockers,
            body,
        );
        body.extend(after_update);
    }

    body.push(
        ctx.b
            .call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&el_name)]),
    );
}

fn emit_single_block<'a>(
    ctx: &mut Ctx<'a>,
    item: &FragmentItem,
    is_root: bool,
    body: &mut Vec<Statement<'a>>,
) {
    // RenderTag / ComponentNode: call directly with $$anchor, no wrapping.
    // Non-root consumes a "fragment" ident for consistent numbering.
    match item {
        FragmentItem::RenderTag(id)
            if !ctx
                .render_tag_plan(*id)
                .unwrap_or_else(|| panic!("render tag plan missing for {:?}", id))
                .callee_mode
                .is_dynamic() =>
        {
            if !is_root {
                ctx.gen_ident("fragment");
            }
            gen_render_tag(ctx, *id, ctx.b.rid_expr("$$anchor"), true, body);
            return;
        }
        FragmentItem::ComponentNode(id) if !ctx.is_dynamic_component(*id) => {
            if !is_root {
                ctx.gen_ident("fragment");
            }
            gen_component(ctx, *id, ctx.b.rid_expr("$$anchor"), body);
            return;
        }
        FragmentItem::Element(_) | FragmentItem::TextConcat { .. } => {
            unreachable!("SingleBlock should not contain Element or TextConcat")
        }
        _ => {}
    }

    let frag = ctx.gen_ident("fragment");
    let node = ctx.gen_ident("node");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr("$.comment", [])));
    body.push(
        ctx.b
            .var_stmt(&node, ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)])),
    );

    match item {
        FragmentItem::IfBlock(id) => {
            let stmts = gen_if_block(ctx, *id, ctx.b.rid_expr(&node));
            if stmts.len() == 1 {
                body.extend(stmts);
            } else {
                body.push(ctx.b.block_stmt(stmts));
            }
        }
        FragmentItem::EachBlock(id) => {
            gen_each_block(ctx, *id, ctx.b.rid_expr(&node), false, body);
        }
        FragmentItem::HtmlTag(id) => {
            gen_html_tag(ctx, *id, ctx.b.rid_expr(&node), false, body);
        }
        FragmentItem::KeyBlock(id) => {
            gen_key_block(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::SvelteElement(id) => {
            gen_svelte_element(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::SvelteBoundary(id) => {
            gen_svelte_boundary(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::AwaitBlock(id) => {
            gen_await_block(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::RenderTag(id) => {
            gen_render_tag(ctx, *id, ctx.b.rid_expr(&node), false, body);
        }
        FragmentItem::ComponentNode(id) => {
            gen_component(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        _ => unreachable!("SingleBlock dispatch: all variants covered above"),
    }

    body.push(
        ctx.b
            .call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]),
    );
}

fn emit_mixed<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    tpl_name: &str,
    is_root: bool,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    // Clone needed: traverse_items borrows ctx mutably
    let items: Vec<_> = ctx.lowered_fragment(&key).items.clone();

    let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
    if key.needs_text_first_next() && starts_text {
        body.push(ctx.b.call_stmt("$.next", []));
    }

    let (html, import_node) = fragment_html(ctx, key.clone());
    let flags = if import_node { 3.0 } else { 1.0 };
    let from_fn = from_template_fn_for_items(ctx, &items);
    let make_tpl_stmt = |ctx: &mut Ctx<'a>, key: FragmentKey| {
        let mut from_html = ctx.b.call_expr(
            from_fn,
            [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(flags)],
        );
        if ctx.state.dev {
            let locs = build_fragment_element_locations(ctx, key);
            from_html = wrap_add_locations(ctx, from_html, locs);
        }
        ctx.b.var_stmt(tpl_name, from_html)
    };

    if is_root {
        // Root: template BEFORE children (top-down)
        hoisted.push(make_tpl_stmt(ctx, key.clone()));
    }

    let frag = ctx.gen_ident("fragment");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr(tpl_name, [])));

    let first_child = ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]);
    let mut init = Vec::new();
    let mut update = Vec::new();
    let mut after_update = Vec::new();
    let mut memo_attrs = Vec::new();
    let trailing = traverse_items(
        ctx,
        &items,
        first_child,
        &mut init,
        &mut update,
        hoisted,
        &mut after_update,
        &mut memo_attrs,
    );

    if !is_root {
        // Non-root: template AFTER children (bottom-up)
        hoisted.push(make_tpl_stmt(ctx, key));
    }

    emit_trailing_next(ctx, trailing, &mut init);
    body.extend(init);
    expression::emit_template_effect_with_memo(ctx, update, memo_attrs, vec![], vec![], body);
    body.extend(after_update);
    body.push(
        ctx.b
            .call_stmt("$.append", [Arg::Ident("$$anchor"), Arg::Ident(&frag)]),
    );
}

// ---------------------------------------------------------------------------
// Dev-mode: $.add_locations wrapping
// ---------------------------------------------------------------------------

/// Wrap a `$.from_html(...)` expression with `$.add_locations(expr, App[$.FILENAME], locs)`.
fn wrap_add_locations<'a>(
    ctx: &mut Ctx<'a>,
    from_html: Expression<'a>,
    locs: Expression<'a>,
) -> Expression<'a> {
    let filename_member = ctx.b.computed_member_expr(
        ctx.b.rid_expr(ctx.state.name),
        ctx.b.static_member_expr(ctx.b.rid_expr("$"), "FILENAME"),
    );
    ctx.b.call_expr(
        "$.add_locations",
        [
            Arg::Expr(from_html),
            Arg::Expr(filename_member),
            Arg::Expr(locs),
        ],
    )
}

/// Build element location arrays for a single element template.
fn build_element_locations<'a>(ctx: &mut Ctx<'a>, el_id: NodeId) -> Expression<'a> {
    let el = ctx.element(el_id);
    let loc = build_single_element_loc(ctx, el.span.start, &el.fragment);
    ctx.b.array_expr([loc])
}

/// Build `[line, col]` or `[line, col, [children...]]` for one element.
fn build_single_element_loc<'a>(
    ctx: &mut Ctx<'a>,
    span_start: u32,
    fragment: &svelte_ast::Fragment,
) -> Expression<'a> {
    let (line, col) = crate::script::compute_line_col(ctx.state.source, span_start);
    let mut inner: Vec<Arg<'a, '_>> = vec![Arg::Num(line as f64), Arg::Num(col as f64)];

    let child_locs = build_child_element_locs(ctx, fragment);
    if !child_locs.is_empty() {
        let child_array = ctx
            .b
            .array_from_args(child_locs.into_iter().map(Arg::Expr).collect::<Vec<_>>());
        inner.push(Arg::Expr(child_array));
    }

    ctx.b.array_from_args(inner)
}

/// Recursively collect location arrays for child elements in a fragment.
fn build_child_element_locs<'a>(
    ctx: &mut Ctx<'a>,
    fragment: &svelte_ast::Fragment,
) -> Vec<Expression<'a>> {
    let mut locs = Vec::new();
    for &id in &fragment.nodes {
        let node = ctx.query.component.store.get(id);
        if let Node::Element(el) = node {
            locs.push(build_single_element_loc(ctx, el.span.start, &el.fragment));
        }
    }
    locs
}

/// Build element location arrays for a mixed fragment's elements.
fn build_fragment_element_locations<'a>(ctx: &mut Ctx<'a>, key: FragmentKey) -> Expression<'a> {
    let items: Vec<_> = ctx.lowered_fragment(&key).items.clone();
    let mut locs: Vec<Expression<'a>> = Vec::new();
    for item in &items {
        if let FragmentItem::Element(el_id) = item {
            let el = ctx.element(*el_id);
            locs.push(build_single_element_loc(ctx, el.span.start, &el.fragment));
        }
    }
    ctx.b
        .array_from_args(locs.into_iter().map(Arg::Expr).collect::<Vec<_>>())
}
