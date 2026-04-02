//! SnippetBlock codegen — `{#snippet name(params)}...{/snippet}`

use oxc_allocator::CloneIn;
use oxc_ast::ast::{
    AssignmentPattern, BindingPattern, ChainElement, Expression, FormalParameters, PropertyKey,
    Statement,
};
use oxc_ast_visit::VisitMut;
use oxc_span::SPAN;
use rustc_hash::FxHashSet;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::gen_fragment;

/// Generate a `const name = ($$anchor, param1 = $.noop, ...) => { ... }` statement.
///
/// `prepend_stmts` are injected before the fragment body (e.g., duplicated @const tags
/// from boundary parents).
pub(crate) fn gen_snippet_block<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    prepend_stmts: Vec<Statement<'a>>,
) -> Statement<'a> {
    let block = ctx.snippet_block(id);
    let name = block.name(ctx.state.source).to_string();

    let parsed_stmt = ctx
        .snippet_stmt_handle(id)
        .and_then(|h| ctx.state.parsed.take_stmt(h));
    let stmt = parsed_stmt.unwrap_or_else(|| {
        panic!("snippet block {:?} has no pre-parsed statement — parser invariant broken", id)
    });

    let mut declarations: Vec<Statement<'a>> = Vec::new();
    let params = build_snippet_params_from_parsed(ctx, &stmt, &mut declarations);
    let body_stmts = gen_fragment(ctx, FragmentKey::SnippetBody(id));

    let mut all_stmts = prepend_stmts;
    if ctx.state.dev {
        let args_id = ctx.b.rid_expr("arguments");
        let validate_stmt = ctx.b.call_stmt("$.validate_snippet_args", [
            Arg::Spread(args_id),
        ]);
        all_stmts.push(validate_stmt);
    }
    all_stmts.extend(declarations);
    all_stmts.extend(body_stmts);

    let snippet_expr = if ctx.state.dev {
        let fn_expr = ctx.b.function_expr(params, all_stmts);
        let component_name = ctx.b.rid_expr(ctx.state.name);
        ctx.b.call_expr("$.wrap_snippet", [
            Arg::Expr(component_name),
            Arg::Expr(fn_expr),
        ])
    } else {
        let arrow = ctx.b.arrow(params, all_stmts);
        oxc_ast::ast::Expression::ArrowFunctionExpression(ctx.b.alloc(arrow))
    };

    ctx.b.const_stmt(&name, snippet_expr)
}

fn build_snippet_params_from_parsed<'stmt, 'a: 'stmt>(
    ctx: &mut Ctx<'a>,
    stmt: &'stmt Statement<'a>,
    decls: &mut Vec<Statement<'a>>,
) -> FormalParameters<'a> {
    use oxc_ast::ast;

    let mut params: Vec<ast::FormalParameter<'a>> = Vec::new();

    let anchor_pattern = ctx
        .b
        .ast
        .binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom("$$anchor"));
    params.push(ctx.b.ast.formal_parameter(
        SPAN,
        ctx.b.ast.vec(),
        anchor_pattern,
        oxc_ast::NONE,
        oxc_ast::NONE,
        false,
        None,
        false,
        false,
    ));

    let Some(items) = extract_arrow_param_items(stmt) else {
        return ctx.b.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::ArrowFormalParameters,
            ctx.b.ast.vec_from_iter(params),
            oxc_ast::NONE,
        );
    };

    for (i, item) in items.iter().enumerate() {
        match &item.pattern {
            BindingPattern::BindingIdentifier(id) => {
                let name = ctx.b.ast.atom(id.name.as_str());
                let default_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "noop");
                let inner = ctx
                    .b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, name);
                let pattern = ctx
                    .b
                    .ast
                    .binding_pattern_assignment_pattern(SPAN, inner, default_expr);
                params.push(ctx.b.ast.formal_parameter(
                    SPAN,
                    ctx.b.ast.vec(),
                    pattern,
                    oxc_ast::NONE,
                    oxc_ast::NONE,
                    false,
                    None,
                    false,
                    false,
                ));
            }
            pattern => {
                let arg_name = format!("$$arg{i}");
                let plain = ctx
                    .b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom(&arg_name));
                params.push(ctx.b.ast.formal_parameter(
                    SPAN,
                    ctx.b.ast.vec(),
                    plain,
                    oxc_ast::NONE,
                    oxc_ast::NONE,
                    false,
                    None,
                    false,
                    false,
                ));

                let access = ctx
                    .b
                    .maybe_call_expr(ctx.b.rid_expr(&arg_name), std::iter::empty::<Arg<'_, '_>>());
                let mut inserts = Vec::new();
                let mut paths = Vec::new();
                collect_binding_pattern(ctx, pattern, access, false, &mut inserts, &mut paths);

                let array_insert_names: FxHashSet<String> =
                    inserts.iter().map(|insert| insert.name.clone()).collect();

                for insert in inserts {
                    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(ctx.b.thunk(insert.value))]);
                    decls.push(ctx.b.var_stmt(&insert.name, derived));
                }

                for mut path in paths {
                    rewrite_array_reads(ctx, &mut path.expression, &array_insert_names);
                    emit_leaf_binding(ctx, &path.name, path.expression, path.has_default_value, decls);
                }
            }
        }
    }

    ctx.b.ast.formal_parameters(
        SPAN,
        ast::FormalParameterKind::ArrowFormalParameters,
        ctx.b.ast.vec_from_iter(params),
        oxc_ast::NONE,
    )
}

fn extract_arrow_param_items<'s, 'a: 's>(
    stmt: &'s Statement<'a>,
) -> Option<&'s [oxc_ast::ast::FormalParameter<'a>]> {
    if let Statement::VariableDeclaration(decl) = stmt {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(oxc_ast::ast::Expression::ArrowFunctionExpression(arrow)) = &declarator.init {
                return Some(arrow.params.items.as_slice());
            }
        }
    }
    None
}

// Snippet params need the same insert-before-path ordering as the reference
// compiler's extract_paths, but we keep the helper local until the other
// destructuring sites are ready to share one abstraction.
struct SnippetInsert<'a> {
    name: String,
    value: Expression<'a>,
}

struct SnippetPath<'a> {
    name: String,
    expression: Expression<'a>,
    has_default_value: bool,
}

fn collect_binding_pattern<'a>(
    ctx: &mut Ctx<'a>,
    pattern: &BindingPattern<'a>,
    expression: Expression<'a>,
    has_default_value: bool,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            paths.push(SnippetPath {
                name: id.name.as_str().to_string(),
                expression,
                has_default_value,
            });
        }
        BindingPattern::ObjectPattern(obj) => {
            collect_object_pattern(ctx, obj, expression, has_default_value, inserts, paths);
        }
        BindingPattern::ArrayPattern(arr) => {
            collect_array_pattern(ctx, arr, expression, has_default_value, inserts, paths);
        }
        BindingPattern::AssignmentPattern(assign) => {
            let fallback = build_fallback_expr(ctx, &expression, assign);
            collect_binding_pattern(ctx, &assign.left, fallback, true, inserts, paths);
        }
    }
}

fn emit_leaf_binding<'a>(
    ctx: &mut Ctx<'a>,
    name: &str,
    access: Expression<'a>,
    has_default_value: bool,
    decls: &mut Vec<Statement<'a>>,
) {
    let init = if has_default_value {
        let thunk = ctx.b.thunk(access);
        ctx.b.call_expr("$.derived_safe_equal", [Arg::Expr(thunk)])
    } else {
        ctx.b.thunk(access)
    };
    decls.push(ctx.b.let_init_stmt(name, init));
}

fn collect_object_pattern<'a>(
    ctx: &mut Ctx<'a>,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
    expression: Expression<'a>,
    has_default_value: bool,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    for prop in &obj.properties {
        let property_access = build_object_property_access(ctx, &expression, prop);
        collect_binding_pattern(ctx, &prop.value, property_access, has_default_value, inserts, paths);
    }

    if let Some(rest) = &obj.rest {
        let rest_expr = build_object_rest_expr(ctx, &expression, obj);
        collect_binding_pattern(
            ctx,
            &rest.argument,
            rest_expr,
            has_default_value,
            inserts,
            paths,
        );
    }
}

fn collect_array_pattern<'a>(
    ctx: &mut Ctx<'a>,
    arr: &oxc_ast::ast::ArrayPattern<'a>,
    expression: Expression<'a>,
    has_default_value: bool,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    let array_name = ctx.gen_ident("$$array");
    let to_array = if arr.rest.is_some() {
        ctx.b.call_expr("$.to_array", [Arg::Expr(expression)])
    } else {
        ctx.b.call_expr(
            "$.to_array",
            [
                Arg::Expr(expression),
                Arg::Num(arr.elements.len() as f64),
            ],
        )
    };
    inserts.push(SnippetInsert {
        name: array_name.clone(),
        value: to_array,
    });

    for (index, element) in arr.elements.iter().enumerate() {
        let Some(pattern) = element else { continue };
        let element_access = build_computed_member_access(
            ctx,
            &ctx.b.rid_expr(&array_name),
            ctx.b.num_expr(index as f64),
        );
        collect_binding_pattern(ctx, pattern, element_access, has_default_value, inserts, paths);
    }

    if let Some(rest) = &arr.rest {
        let rest_access = ctx.b.call_expr_callee(
            build_static_member_access(ctx, &ctx.b.rid_expr(&array_name), "slice"),
            [Arg::Num(arr.elements.len() as f64)],
        );
        collect_binding_pattern(
            ctx,
            &rest.argument,
            rest_access,
            has_default_value,
            inserts,
            paths,
        );
    }
}

fn build_fallback_expr<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    assign: &AssignmentPattern<'a>,
) -> oxc_ast::ast::Expression<'a> {
    let default_val = assign.right.clone_in(ctx.b.ast.allocator);
    ctx.b.call_expr(
        "$.fallback",
        [Arg::Expr(ctx.b.clone_expr(access)), Arg::Expr(default_val)],
    )
}

fn build_object_property_access<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    prop: &oxc_ast::ast::BindingProperty<'a>,
) -> oxc_ast::ast::Expression<'a> {
    if !prop.computed {
        if let PropertyKey::StaticIdentifier(id) = &prop.key {
            return build_static_member_access(ctx, access, id.name.as_str());
        }
    }

    build_computed_member_access(ctx, access, clone_property_key_expr(ctx, &prop.key))
}

fn build_object_rest_expr<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
) -> oxc_ast::ast::Expression<'a> {
    let excluded = ctx.b.array_expr(
        obj.properties
            .iter()
            .map(|prop| excluded_key_expr(ctx, prop)),
    );
    ctx.b.call_expr(
        "$.exclude_from_object",
        [Arg::Expr(ctx.b.clone_expr(access)), Arg::Expr(excluded)],
    )
}

fn excluded_key_expr<'a>(
    ctx: &Ctx<'a>,
    prop: &oxc_ast::ast::BindingProperty<'a>,
) -> oxc_ast::ast::Expression<'a> {
    match &prop.key {
        PropertyKey::StaticIdentifier(id) if !prop.computed => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::StringLiteral(str_) => ctx.b.str_expr(str_.value.as_str()),
        PropertyKey::NumericLiteral(num) => ctx.b.str_expr(&num.value.to_string()),
        key if prop.computed => {
            ctx.b.call_expr("String", [Arg::Expr(clone_property_key_expr(ctx, key))])
        }
        _ => ctx.b.call_expr("String", [Arg::Expr(clone_property_key_expr(ctx, &prop.key))]),
    }
}

fn clone_property_key_expr<'a>(
    ctx: &Ctx<'a>,
    key: &PropertyKey<'a>,
) -> oxc_ast::ast::Expression<'a> {
    match key {
        PropertyKey::StaticIdentifier(id) => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::PrivateIdentifier(id) => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::BooleanLiteral(it) => Expression::BooleanLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::NullLiteral(it) => Expression::NullLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::NumericLiteral(it) => Expression::NumericLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::BigIntLiteral(it) => Expression::BigIntLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::RegExpLiteral(it) => Expression::RegExpLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::StringLiteral(it) => Expression::StringLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::TemplateLiteral(it) => Expression::TemplateLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::Identifier(it) => Expression::Identifier(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::MetaProperty(it) => Expression::MetaProperty(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::Super(it) => Expression::Super(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ArrayExpression(it) => Expression::ArrayExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ArrowFunctionExpression(it) => {
            Expression::ArrowFunctionExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::AssignmentExpression(it) => {
            Expression::AssignmentExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::AwaitExpression(it) => Expression::AwaitExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::BinaryExpression(it) => Expression::BinaryExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::CallExpression(it) => Expression::CallExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ChainExpression(it) => Expression::ChainExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ClassExpression(it) => Expression::ClassExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ConditionalExpression(it) => {
            Expression::ConditionalExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::FunctionExpression(it) => {
            Expression::FunctionExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ImportExpression(it) => Expression::ImportExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::LogicalExpression(it) => Expression::LogicalExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::NewExpression(it) => Expression::NewExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ObjectExpression(it) => Expression::ObjectExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ParenthesizedExpression(it) => {
            Expression::ParenthesizedExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::SequenceExpression(it) => Expression::SequenceExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::TaggedTemplateExpression(it) => {
            Expression::TaggedTemplateExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ThisExpression(it) => Expression::ThisExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::UnaryExpression(it) => Expression::UnaryExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::UpdateExpression(it) => Expression::UpdateExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::YieldExpression(it) => Expression::YieldExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::PrivateInExpression(it) => {
            Expression::PrivateInExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::JSXElement(it) => Expression::JSXElement(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::JSXFragment(it) => Expression::JSXFragment(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::TSAsExpression(it) => Expression::TSAsExpression(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::TSSatisfiesExpression(it) => {
            Expression::TSSatisfiesExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSTypeAssertion(it) => {
            Expression::TSTypeAssertion(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSNonNullExpression(it) => {
            Expression::TSNonNullExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSInstantiationExpression(it) => {
            Expression::TSInstantiationExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ComputedMemberExpression(it) => {
            Expression::ComputedMemberExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::StaticMemberExpression(it) => {
            Expression::StaticMemberExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::PrivateFieldExpression(it) => {
            Expression::PrivateFieldExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::V8IntrinsicExpression(it) => {
            Expression::V8IntrinsicExpression(it.clone_in(ctx.b.ast.allocator))
        }
    }
}

fn build_static_member_access<'a>(
    ctx: &Ctx<'a>,
    object: &Expression<'a>,
    prop: &str,
) -> Expression<'a> {
    if let Expression::ChainExpression(chain) = object {
        let property = ctx.b.ast.identifier_name(SPAN, ctx.b.ast.atom(prop));
        let member = ctx.b.ast.static_member_expression(
            SPAN,
            clone_chain_element_expr(ctx, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(ctx.b.alloc(
            ctx.b.ast.chain_expression(
                SPAN,
                ChainElement::StaticMemberExpression(ctx.b.alloc(member)),
            ),
        ));
    }

    ctx.b.static_member_expr(ctx.b.clone_expr(object), prop)
}

fn build_computed_member_access<'a>(
    ctx: &Ctx<'a>,
    object: &Expression<'a>,
    property: Expression<'a>,
) -> Expression<'a> {
    if let Expression::ChainExpression(chain) = object {
        let member = ctx.b.ast.computed_member_expression(
            SPAN,
            clone_chain_element_expr(ctx, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(ctx.b.alloc(
            ctx.b.ast.chain_expression(
                SPAN,
                ChainElement::ComputedMemberExpression(ctx.b.alloc(member)),
            ),
        ));
    }

    ctx.b.computed_member_expr(ctx.b.clone_expr(object), property)
}

fn clone_chain_element_expr<'a>(ctx: &Ctx<'a>, element: &ChainElement<'a>) -> Expression<'a> {
    match element {
        ChainElement::CallExpression(call) => Expression::CallExpression(call.clone_in(ctx.b.ast.allocator)),
        ChainElement::StaticMemberExpression(member) => {
            Expression::StaticMemberExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::ComputedMemberExpression(member) => {
            Expression::ComputedMemberExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::PrivateFieldExpression(member) => {
            Expression::PrivateFieldExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::TSNonNullExpression(expr) => {
            Expression::TSNonNullExpression(expr.clone_in(ctx.b.ast.allocator))
        }
    }
}

fn rewrite_array_reads<'a>(
    ctx: &Ctx<'a>,
    expr: &mut Expression<'a>,
    array_insert_names: &FxHashSet<String>,
) {
    let mut rewriter = ArrayReadRewriter {
        ctx,
        array_insert_names,
    };
    rewriter.visit_expression(expr);
}

struct ArrayReadRewriter<'c, 'a> {
    ctx: &'c Ctx<'a>,
    array_insert_names: &'c FxHashSet<String>,
}

impl<'a> VisitMut<'a> for ArrayReadRewriter<'_, 'a> {
    fn visit_expression(&mut self, expr: &mut Expression<'a>) {
        oxc_ast_visit::walk_mut::walk_expression(self, expr);

        let Expression::Identifier(ident) = expr else {
            return;
        };

        if !self.array_insert_names.contains(ident.name.as_str()) {
            return;
        }

        *expr = self
            .ctx
            .b
            .call_expr("$.get", [Arg::Ident(ident.name.as_str())]);
    }
}
