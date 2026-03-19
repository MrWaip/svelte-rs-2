use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey, Statement};
use oxc_span::GetSpan as _;
use svelte_ast::CustomElementConfig;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Parsed custom element config (from object form expression)
// ---------------------------------------------------------------------------

struct CePropDef {
    attribute: Option<String>,
    reflect: bool,
    prop_type: Option<String>,
}

struct ParsedCeOptions {
    tag: Option<String>,
    shadow: ShadowMode,
    /// Ordered list of (prop_name, prop_def) pairs, preserving config order.
    props: Vec<(String, CePropDef)>,
    /// Source text of the `extend` expression (re-emitted as-is).
    extend_source: Option<String>,
}

#[derive(PartialEq)]
enum ShadowMode {
    Open,
    None,
}

// ---------------------------------------------------------------------------
// Object form parsing
// ---------------------------------------------------------------------------

/// Parse a `CustomElementConfig::Expression` span into structured options.
/// Re-parses the source slice with OXC and walks the ObjectExpression.
fn parse_ce_expression(source: &str, span: svelte_span::Span) -> ParsedCeOptions {
    let start = span.start as usize;
    let end = span.end as usize;
    let expr_text = &source[start..end];

    let alloc = oxc_allocator::Allocator::default();
    let arena_text: &str = alloc.alloc_str(expr_text);
    let parsed = oxc_parser::Parser::new(&alloc, arena_text, oxc_span::SourceType::default())
        .parse_expression();

    let expr = match parsed {
        Ok(expr) => expr,
        Err(_) => return ParsedCeOptions {
            tag: None,
            shadow: ShadowMode::Open,
            props: Vec::new(),
            extend_source: None,
        },
    };

    let mut tag = None;
    let mut shadow = ShadowMode::Open;
    let mut props = Vec::new();
    let mut extend_source = None;

    if let Expression::ObjectExpression(obj) = &expr {
        for prop_kind in &obj.properties {
            let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else { continue };
            let key_name = match &prop.key {
                PropertyKey::StaticIdentifier(id) => id.name.as_str(),
                _ => continue,
            };

            match key_name {
                "tag" => {
                    if let Expression::StringLiteral(lit) = &prop.value {
                        tag = Some(lit.value.to_string());
                    }
                }
                "shadow" => {
                    if let Expression::StringLiteral(lit) = &prop.value {
                        match lit.value.as_str() {
                            "none" => shadow = ShadowMode::None,
                            _ => shadow = ShadowMode::Open,
                        }
                    }
                }
                "props" => {
                    if let Expression::ObjectExpression(props_obj) = &prop.value {
                        for prop_entry in &props_obj.properties {
                            let ObjectPropertyKind::ObjectProperty(entry) = prop_entry else { continue };
                            let prop_name = match &entry.key {
                                PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                                _ => continue,
                            };
                            let mut def = CePropDef {
                                attribute: None,
                                reflect: false,
                                prop_type: None,
                            };
                            if let Expression::ObjectExpression(def_obj) = &entry.value {
                                for def_prop in &def_obj.properties {
                                    let ObjectPropertyKind::ObjectProperty(dp) = def_prop else { continue };
                                    let dk = match &dp.key {
                                        PropertyKey::StaticIdentifier(id) => id.name.as_str(),
                                        _ => continue,
                                    };
                                    match dk {
                                        "attribute" => {
                                            if let Expression::StringLiteral(lit) = &dp.value {
                                                def.attribute = Some(lit.value.to_string());
                                            }
                                        }
                                        "reflect" => {
                                            if let Expression::BooleanLiteral(lit) = &dp.value {
                                                def.reflect = lit.value;
                                            }
                                        }
                                        "type" => {
                                            if let Expression::StringLiteral(lit) = &dp.value {
                                                def.prop_type = Some(lit.value.to_string());
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            props.push((prop_name, def));
                        }
                    }
                }
                "extend" => {
                    let ext_start = prop.value.span().start as usize;
                    let ext_end = prop.value.span().end as usize;
                    extend_source = Some(expr_text[ext_start..ext_end].to_string());
                }
                _ => {}
            }
        }
    }

    ParsedCeOptions { tag, shadow, props, extend_source }
}

// ---------------------------------------------------------------------------
// Codegen
// ---------------------------------------------------------------------------

/// Generate the `customElements.define(tag, $.create_custom_element(...))` statement(s).
pub fn gen_custom_element<'a>(
    ctx: &Ctx<'a>,
    ce_config: &CustomElementConfig,
) -> Vec<Statement<'a>> {
    let b = &ctx.b;

    // Determine tag and parsed options based on config variant
    let (simple_tag, parsed) = match ce_config {
        CustomElementConfig::Tag(tag) => (Some(tag.as_str()), None),
        CustomElementConfig::Expression(span) => {
            let opts = parse_ce_expression(ctx.source, *span);
            (None, Some(opts))
        }
    };

    // Resolve tag: simple form uses tag directly, object form uses parsed tag
    let resolved_tag: Option<&str> = match (&simple_tag, &parsed) {
        (Some(t), _) => Some(t),
        (None, Some(opts)) => opts.tag.as_deref(),
        (None, None) => None,
    };

    let parsed_opts = parsed.as_ref();

    // -- Arg 2: Props metadata object --
    let props_obj = build_props_metadata(ctx, parsed_opts);

    // -- Arg 3: Slots array (always empty in Svelte 5 runes mode) --
    let slots = b.array_from_args(std::iter::empty::<Arg<'_, '_>>());

    // -- Arg 4: Accessors array (from exports) --
    let accessors = b.array_from_args(
        ctx.analysis.exports.iter().map(|e| {
            let name = e.alias.as_deref().unwrap_or(e.name.as_str());
            Arg::Str(name.to_string())
        })
    );

    // -- Arg 5: Shadow root config --
    let is_shadow_none = parsed_opts.is_some_and(|o| o.shadow == ShadowMode::None);

    // -- Arg 6: Extend (optional) --
    let extend_arg: Option<Expression<'a>> = parsed_opts
        .and_then(|o| o.extend_source.as_deref())
        .map(|src| b.parse_expression(src));

    // Build $.create_custom_element() call
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(ctx.name),
        Arg::Expr(props_obj),
        Arg::Expr(slots),
        Arg::Expr(accessors),
    ];

    if !is_shadow_none {
        args.push(Arg::Expr(
            b.object_expr([ObjProp::KeyValue("mode", b.str_expr("open"))]),
        ));
    }

    if let Some(extend_expr) = extend_arg {
        args.push(Arg::Expr(extend_expr));
    }

    let create_ce = b.call_expr("$.create_custom_element", args);

    // Wrap in customElements.define() if tag is present
    let mut stmts = Vec::new();
    if let Some(tag_str) = resolved_tag {
        let define_callee = b.static_member_expr(
            b.rid_expr("customElements"),
            "define",
        );
        let define_call = b.call_expr_callee(define_callee, [
            Arg::Str(tag_str.to_string()),
            Arg::Expr(create_ce),
        ]);
        stmts.push(b.expr_stmt(define_call));
    } else {
        stmts.push(b.expr_stmt(create_ce));
    }

    stmts
}

/// Build the props metadata object for `$.create_custom_element()`.
///
/// For each prop from CE config: build `{ attribute?, reflect?, type? }`.
/// For remaining component props not in config: add `propName: {}`.
fn build_props_metadata<'a>(
    ctx: &Ctx<'a>,
    parsed_opts: Option<&ParsedCeOptions>,
) -> Expression<'a> {
    let b = &ctx.b;
    let mut obj_props: Vec<ObjProp<'a>> = Vec::new();

    // Collect CE config prop names for dedup check
    let ce_prop_names: Vec<&str> = parsed_opts
        .map(|o| o.props.iter().map(|(n, _)| n.as_str()).collect())
        .unwrap_or_default();

    // First: emit props from CE config (preserving config order)
    if let Some(opts) = parsed_opts {
        for (name, def) in &opts.props {
            let prop_key = resolve_prop_key(ctx, name);
            let value = build_prop_def_expr(b, def);
            obj_props.push(ObjProp::KeyValue(b.alloc_str(&prop_key), value));
        }
    }

    // Second: emit remaining component props not already in CE config
    if let Some(ref props_analysis) = ctx.analysis.props {
        for prop in &props_analysis.props {
            if prop.is_rest || prop.prop_name.starts_with("$$") {
                continue;
            }
            let key = &prop.prop_name;
            // Skip if already covered by CE config
            if ce_prop_names.iter().any(|&n| n == key) {
                continue;
            }
            obj_props.push(ObjProp::KeyValue(
                b.alloc_str(key),
                b.object_expr(std::iter::empty::<ObjProp<'_>>()),
            ));
        }
    }

    b.object_expr(obj_props)
}

/// Resolve the prop key: use prop_alias if the binding has one, otherwise use the name.
fn resolve_prop_key(ctx: &Ctx<'_>, name: &str) -> String {
    if let Some(ref props) = ctx.analysis.props {
        for prop in &props.props {
            if prop.local_name == name || prop.prop_name == name {
                return prop.prop_name.clone();
            }
        }
    }
    name.to_string()
}

/// Build the value expression for a single prop definition: `{ attribute?, reflect?, type? }`.
fn build_prop_def_expr<'a>(
    b: &crate::builder::Builder<'a>,
    def: &CePropDef,
) -> Expression<'a> {
    let mut props: Vec<ObjProp<'a>> = Vec::new();

    if let Some(ref attr) = def.attribute {
        props.push(ObjProp::KeyValue("attribute", b.str_expr(attr)));
    }
    if def.reflect {
        props.push(ObjProp::KeyValue("reflect", b.bool_expr(true)));
    }
    if let Some(ref typ) = def.prop_type {
        props.push(ObjProp::KeyValue("type", b.str_expr(typ)));
    }

    b.object_expr(props)
}
