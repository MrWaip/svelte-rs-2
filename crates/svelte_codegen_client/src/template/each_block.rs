//! EachBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::{Attribute, Node, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_node_expr;
use super::gen_fragment;

// Each block flags (from Svelte constants)
const EACH_ITEM_REACTIVE: u32 = 1;
const EACH_INDEX_REACTIVE: u32 = 2;
const EACH_IS_CONTROLLED: u32 = 4;
const EACH_IS_ANIMATED: u32 = 8;
const EACH_ITEM_IMMUTABLE: u32 = 16;

/// Generate statements for an `{#each ...}` block.
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    is_controlled: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.each_block(block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let context_span = block.context_span;
    let span_start = block.span.start;
    let has_key = block.key_span.is_some();
    let has_index = block.index_span.is_some();
    let has_fallback = block.fallback.is_some();
    let is_destructured = ctx.each_is_destructured(block_id);
    let context_name = ctx.each_context_name(block_id);

    // key_is_item: key expression equals context identifier (both simple identifiers)
    let key_is_item = block.key_span.map_or(false, |key_span| {
        let key_text = ctx.component.source_text(key_span).trim();
        let ctx_text = ctx.component.source_text(context_span).trim();
        key_text == ctx_text
            && !ctx_text.starts_with('{')
            && !ctx_text.starts_with('[')
            && is_simple_identifier(key_text)
    });

    // Compute flags
    let mut flags: u32 = 0;

    // EACH_ITEM_IMMUTABLE: runes mode without store (always set for now)
    flags |= EACH_ITEM_IMMUTABLE;

    // EACH_INDEX_REACTIVE: keyed block with user-declared index
    if has_key && has_index {
        flags |= EACH_INDEX_REACTIVE;
    }

    // EACH_ITEM_REACTIVE: collection references external state
    // In runes mode: skip when key_is_item (item identity is the key)
    let expr_has_refs = ctx.expression(block_id)
        .is_some_and(|info| !info.references.is_empty());
    if expr_has_refs && !key_is_item {
        flags |= EACH_ITEM_REACTIVE;
    }

    if is_controlled {
        flags |= EACH_IS_CONTROLLED;
    }

    // animate: can only appear on elements that are the sole child of a keyed each block
    if has_key && block.body.nodes.iter().any(|node| {
        if let Node::Element(el) = node {
            el.attributes.iter().any(|a| matches!(a, Attribute::AnimateDirective(_)))
        } else {
            false
        }
    }) {
        flags |= EACH_IS_ANIMATED;
    }

    // User-declared index name (always available regardless of body usage)
    let user_index_name = ctx.each_index_name(block_id);

    // Render function index param: only when body uses it or group binding needs it
    let needs_group_index = ctx.contains_group_binding(block_id);
    let body_uses_index = ctx.each_body_uses_index(block_id);
    let render_index_name = if body_uses_index || needs_group_index {
        user_index_name.clone()
            .or_else(|| needs_group_index.then(|| {
                let name = ctx.gen_ident("$$index");
                ctx.group_index_names.insert(block_id, name.clone());
                name
            }))
    } else {
        None
    };

    // Pre-computed by analysis — no string-based symbol re-resolution.
    let is_prop_source = ctx.is_prop_source_node(block_id);
    let collection_fn = if is_prop_source {
        // Prop getter is already a function — pass directly without thunk
        let expr_source = ctx.component.source_text(expr_span).trim();
        ctx.b.rid_expr(expr_source)
    } else {
        let collection = get_node_expr(ctx, block_id);
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(collection)])
    };

    // Key function: keyed each uses (item[, index]) => key_expr, unkeyed uses $.index
    let key_fn = if let Some(key_expr) = ctx.parsed.key_exprs.remove(&block_id) {
        let key_body = ctx.b.expr_stmt(key_expr);
        if ctx.each_key_uses_index(block_id) {
            let idx_name = user_index_name.as_ref()
                .expect("key_uses_index implies index exists");
            ctx.b.arrow_expr(ctx.b.params([&context_name, idx_name]), [key_body])
        } else {
            ctx.b.arrow_expr(ctx.b.params([&context_name]), [key_body])
        }
    } else {
        ctx.b.rid_expr("$.index")
    };

    // Destructuring declarations prepended to body
    let item_reactive = flags & EACH_ITEM_REACTIVE != 0;
    let destructured_decls = if is_destructured {
        gen_destructuring_declarations(ctx, block_id, item_reactive)
    } else {
        Vec::new()
    };

    let mut frag_body = gen_fragment(ctx, body_key);

    // Prepend destructuring declarations
    if !destructured_decls.is_empty() {
        let mut combined = destructured_decls;
        combined.append(&mut frag_body);
        frag_body = combined;
    }

    let frag_fn = if let Some(ref idx) = render_index_name {
        ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", &context_name, idx]), frag_body)
    } else {
        ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", &context_name]), frag_body)
    };

    let mut each_args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(anchor),
        Arg::Num(flags as f64),
        Arg::Expr(collection_fn),
        Arg::Expr(key_fn),
        Arg::Expr(frag_fn),
    ];

    // Fallback: 6th arg — `($$anchor) => { ...fallback_body }`
    if has_fallback {
        let fallback_key = FragmentKey::EachFallback(block_id);
        let fallback_body = gen_fragment(ctx, fallback_key);
        let fallback_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), fallback_body);
        each_args.push(Arg::Expr(fallback_fn));
    }

    let each_call = ctx.b.call_expr("$.each", each_args);
    body.push(super::add_svelte_meta(ctx, each_call, span_start, "each"));
}

fn is_simple_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Generate destructuring declarations for `{#each items as { name, value }}` patterns.
///
/// Object `{ a, b }` → `let a = () => $.get($$item).a;` (thunks)
/// Object `{ a = 5 }` → `let a = $.derived_safe_equal(() => $.fallback($.get($$item).a, 5));`
/// Array `[a, b]` → intermediate `$.derived(() => $.to_array($.get($$item), N))` + per-element thunks
fn gen_destructuring_declarations<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    item_reactive: bool,
) -> Vec<Statement<'a>> {
    let block = ctx.each_block(block_id);
    let pattern_text = ctx.component.source_text(block.context_span);
    let pattern_trimmed = pattern_text.trim();
    let is_array = pattern_trimmed.starts_with('[');

    let bindings = svelte_analyze::scope::extract_destructuring_bindings(pattern_text);
    let mut decls = Vec::new();

    if is_array {
        // Array destructuring: intermediate $$array derived
        let count = bindings.len();
        let array_name = ctx.gen_ident("$$array");

        // $.get($$item) or $$item
        let item_access = if item_reactive {
            ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
        } else {
            ctx.b.rid_expr("$$item")
        };

        // $.to_array($.get($$item), N)
        let to_array = ctx.b.call_expr("$.to_array", [
            Arg::Expr(item_access),
            Arg::Num(count as f64),
        ]);

        // var $$array = $.derived(() => $.to_array(...))
        let thunk = ctx.b.thunk(to_array);
        let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
        decls.push(ctx.b.var_stmt(&array_name, derived));

        // Per-element thunks: let key = () => $.get($$array)[0]
        for (i, (name, _has_default)) in bindings.iter().enumerate() {
            let get_array = ctx.b.call_expr("$.get", [Arg::Ident(&array_name)]);
            let index_expr = ctx.b.num_expr(i as f64);
            let access = ctx.b.computed_member_expr(get_array, index_expr);
            let thunk = ctx.b.thunk(access);
            decls.push(ctx.b.let_init_stmt(name, thunk));
        }
    } else {
        // Object destructuring
        for (name, has_default) in &bindings {
            // $.get($$item) or $$item
            let item_access = if item_reactive {
                ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
            } else {
                ctx.b.rid_expr("$$item")
            };

            // $.get($$item).name
            let member = ctx.b.static_member_expr(item_access, name);

            if *has_default {
                // Extract default value text from pattern
                let default_text = extract_default_value(pattern_text, name);

                // $.fallback($.get($$item).name, default)
                let default_expr = parse_inline_expr(ctx, &default_text);
                let fallback = ctx.b.call_expr("$.fallback", [
                    Arg::Expr(member),
                    Arg::Expr(default_expr),
                ]);

                // $.derived_safe_equal(() => $.fallback(...))
                let thunk = ctx.b.thunk(fallback);
                let derived = ctx.b.call_expr("$.derived_safe_equal", [Arg::Expr(thunk)]);
                decls.push(ctx.b.let_init_stmt(name, derived));
            } else {
                // let name = () => $.get($$item).name
                let thunk = ctx.b.thunk(member);
                decls.push(ctx.b.let_init_stmt(name, thunk));
            }
        }
    }

    decls
}

/// Extract the default value text for a named field from a destructuring pattern.
/// E.g., from `{ name, value = 'N/A' }`, extract `'N/A'` for field `value`.
fn extract_default_value(pattern: &str, field_name: &str) -> String {
    let s = pattern.trim();
    let inner = if s.starts_with('{') && s.ends_with('}') {
        &s[1..s.len() - 1]
    } else if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        return String::new();
    };

    // Split by commas at depth 0
    let mut depth = 0i32;
    let mut current = String::new();
    let mut parts = Vec::new();

    for ch in inner.chars() {
        match ch {
            '{' | '[' | '(' => { depth += 1; current.push(ch); }
            '}' | ']' | ')' => { depth -= 1; current.push(ch); }
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    for part in &parts {
        let part = part.trim();
        // Find field name followed by '='
        if let Some(eq_idx) = part.find('=') {
            let name_part = part[..eq_idx].trim();
            // Handle rename: `prop: alias = default`
            let name_part = if let Some(colon_idx) = name_part.rfind(':') {
                name_part[colon_idx + 1..].trim()
            } else {
                name_part
            };
            if name_part == field_name {
                return part[eq_idx + 1..].trim().to_string();
            }
        }
    }

    String::new()
}

/// Parse a short inline expression string using the OXC parser available in builder.
fn parse_inline_expr<'a>(ctx: &mut Ctx<'a>, source: &str) -> Expression<'a> {
    let arena_source = ctx.b.alloc_str(source);
    let parser = oxc_parser::Parser::new(ctx.b.ast.allocator, arena_source, oxc_span::SourceType::default());
    match parser.parse_expression() {
        Ok(expr) => expr,
        Err(_) => ctx.b.str_expr(source),
    }
}
