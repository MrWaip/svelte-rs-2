use std::collections::HashMap;
use std::mem;

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, CallExpression, Expression, Statement, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_span::{GetSpan, SPAN};

use svelte_analyze::{DerivedKind, DerivedSource};

use svelte_ast_builder::{Arg, AssignLeft, Builder};
use svelte_component_semantics::{SymbolId, walk_bindings};

use super::super::location::sanitize_location;
use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_derived(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        derived_kind: DerivedKind,
        source: DerivedSource,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            self.rewrite_single_identifier_derived(declarator, derived_kind, source, out);
            return;
        }

        let init = declarator
            .init
            .take()
            .expect("$derived destructure declarator carries an initializer");
        let Expression::CallExpression(mut call) = init else {
            unreachable!("sync $derived destructure initializer is a call");
        };
        call.callee = self.b.rid_expr("$.derived");

        let inline = matches!(derived_kind, DerivedKind::Derived)
            && call
                .arguments
                .first()
                .and_then(|arg| arg.as_expression())
                .is_some_and(derived_source_is_reference);

        let root_template: Expression<'a> = if inline {
            if self.derived_source_is_whole_props(&call) {
                self.b.rid_expr("$$props")
            } else {
                call.arguments.remove(0).into_expression()
            }
        } else {
            let arg = call.arguments.remove(0).into_expression();
            let derived_arg = if matches!(derived_kind, DerivedKind::DerivedBy) {
                arg
            } else {
                let thunk = self.b.thunk(arg);
                self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);
                thunk
            };
            call.arguments.push(Argument::from(derived_arg));
            let derived_call = Expression::CallExpression(call);
            self.push_boxed_temp(derived_call, decl_kind, out)
        };

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let leaves = self.collect_derived_leaves(
            &declarator.id,
            &root_template,
            decl_kind,
            &mut carriers,
            &mut carrier_declarators,
        );

        out.extend(carrier_declarators);
        for (symbol, access) in leaves {
            out.push(self.derived_leaf_declarator(symbol, access, decl_kind));
        }
    }

    pub(super) fn rewrite_async_derived(
        &mut self,
        decl_kind: VariableDeclarationKind,
        span_start: u32,
        mut declarator: VariableDeclarator<'a>,
    ) -> Statement<'a> {
        let init = declarator
            .init
            .take()
            .expect("async $derived destructure declarator carries an initializer");
        let dev_label = match &declarator.id {
            BindingPattern::ArrayPattern(_) => "[$derived iterable]",
            _ => "[$derived object]",
        };
        let tmp_init = self.async_derived_init(init, dev_label, span_start);
        let tmp_name = self.ident_gen.generate("$$d");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);
        let root = self.b.call_expr("$.get", [Arg::Ident(tmp_name_str)]);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let leaves = self.collect_derived_leaves(
            &declarator.id,
            &root,
            decl_kind,
            &mut carriers,
            &mut carrier_declarators,
        );

        if self.function_info_stack.is_empty() {
            let mut block: Vec<Statement<'a>> = vec![self.b.var_stmt(tmp_name_str, tmp_init)];
            for carrier in carrier_declarators {
                block.push(self.b.var_init_stmt(carrier));
            }
            for (symbol, access) in leaves {
                let (name, value) = self.derived_leaf_value(symbol, access);
                block.push(
                    self.b
                        .assign_stmt(AssignLeft::Ident(name.to_string()), value),
                );
            }
            let block_stmt = self.b.block_stmt(block);
            self.b.seed_block_scope(&block_stmt, self.gen_arrow_scope);
            block_stmt
        } else {
            let mut decls: OxcVec<'a, VariableDeclarator<'a>> = self.b.ast.vec();
            decls.push(
                self.b.ast.variable_declarator(
                    SPAN,
                    decl_kind,
                    self.b
                        .ast
                        .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
                    NONE,
                    Some(tmp_init),
                    false,
                ),
            );
            for carrier in carrier_declarators {
                decls.push(carrier);
            }
            for (symbol, access) in leaves {
                decls.push(self.derived_leaf_declarator(symbol, access, decl_kind));
            }
            let decl = self
                .b
                .ast
                .variable_declaration(SPAN, decl_kind, decls, false);
            Statement::VariableDeclaration(self.b.alloc(decl))
        }
    }

    fn rewrite_single_identifier_derived(
        &mut self,
        mut declarator: VariableDeclarator<'a>,
        derived_kind: DerivedKind,
        source: DerivedSource,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
            unreachable!("single-identifier derived declarator");
        };
        let binding_name: &'a str = self.b.alloc_str(binding.name.as_str());

        let init = declarator
            .init
            .take()
            .expect("$derived declarator carries an initializer");
        let Expression::CallExpression(mut call) = init else {
            unreachable!("sync $derived initializer is a call");
        };
        let callee_span = call.callee.span();
        call.callee = self.b.rid_expr_at("$.derived", callee_span);

        if matches!(derived_kind, DerivedKind::Derived) {
            let passthrough = matches!(source, DerivedSource::Passthrough);
            let mut dummy = Argument::from(self.b.cheap_expr());
            mem::swap(&mut call.arguments[0], &mut dummy);
            let arg = dummy.into_expression().into_inner_expression();
            let wrapped = if passthrough && matches!(arg, Expression::Identifier(_)) {
                let Expression::Identifier(id) = &arg else {
                    unreachable!();
                };
                self.b.rid_expr(id.name.as_str())
            } else {
                let thunk = self.b.thunk(arg);
                self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);
                thunk
            };
            call.arguments[0] = Argument::from(wrapped);
        }

        let derived_expr = Expression::CallExpression(call);
        declarator.init = Some(if self.dev {
            self.b.call_expr(
                "$.tag",
                [Arg::Expr(derived_expr), Arg::StrRef(binding_name)],
            )
        } else {
            derived_expr
        });
        out.push(declarator);
    }

    pub(super) fn rewrite_single_identifier_async_derived(
        &mut self,
        span_start: u32,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
            unreachable!("single-identifier async derived declarator");
        };
        let var_name: &'a str = self.b.alloc_str(binding.name.as_str());

        let init = declarator
            .init
            .take()
            .expect("async $derived declarator carries an initializer");
        let Expression::CallExpression(mut call) = init else {
            unreachable!("async $derived initializer is a call");
        };
        let init_span_start = call.span.start;
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        let awaited = dummy.into_expression().into_inner_expression();

        let track_inner_await = self.dev
            && !self
                .ignore_query
                .is_ignored_at_span(span_start, "await_reactivity_loss");
        let thunk = if let Expression::AwaitExpression(await_expr) = awaited {
            let source_expr = await_expr.unbox().argument;
            if track_inner_await {
                let await_inner = self.b.await_expr(source_expr);
                self.b.async_arrow_expr_body(await_inner)
            } else {
                self.b.thunk(source_expr)
            }
        } else {
            self.b.async_arrow_expr_body(awaited)
        };
        self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(thunk)];
        if self.dev {
            args.push(Arg::Expr(self.b.str_expr(var_name)));
            if !self
                .ignore_query
                .is_ignored_at_span(span_start, "await_waterfall")
            {
                let (line, col) = self.component_line_index.line_col(init_span_start);
                let loc = format!("{}:{}:{}", sanitize_location(self.filename), line, col);
                args.push(Arg::Expr(self.b.str_expr(&loc)));
            }
        }

        let async_derived = self.b.call_expr("$.async_derived", args);
        declarator.init = Some(self.b.await_expr(async_derived));
        out.push(declarator);
    }

    fn wrap_derived_value(&self, value: Expression<'a>) -> Expression<'a> {
        let value = value.into_inner_expression();
        let thunk = self
            .b
            .arrow_expr(self.b.no_params(), [self.b.expr_stmt(value)]);
        self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);
        self.b.call_expr("$.derived", [Arg::Expr(thunk)])
    }

    fn collect_derived_leaves(
        &mut self,
        pattern: &BindingPattern<'a>,
        root_template: &Expression<'a>,
        decl_kind: VariableDeclarationKind,
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
    ) -> Vec<(SymbolId, Expression<'a>)> {
        let mut leaves = Vec::new();
        let carrier_label = match pattern {
            BindingPattern::ArrayPattern(_) => "[$derived iterable]",
            _ => "[$derived object]",
        };
        walk_bindings(pattern, |v| {
            let root = root_template.clone_in_with_semantic_ids(self.b.ast.allocator);
            let access = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                carriers,
                carrier_declarators,
                Some(carrier_label),
                decl_kind,
            );
            leaves.push((v.symbol, access));
        });
        leaves
    }

    fn async_derived_init(
        &mut self,
        init: Expression<'a>,
        dev_label: &str,
        span_start: u32,
    ) -> Expression<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("async $derived initializer is a call");
        };
        let init_span_start = call.span.start;
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        let awaited = dummy.into_expression();

        let track_inner_await = self.dev
            && !self
                .ignore_query
                .is_ignored_at_span(span_start, "await_reactivity_loss");
        let thunk = if let Expression::AwaitExpression(await_expr) = awaited {
            let source_expr = await_expr.unbox().argument;
            let await_inner = self.b.await_expr(source_expr);
            if track_inner_await {
                self.b.async_arrow_expr_body(await_inner)
            } else {
                self.b.async_thunk(await_inner)
            }
        } else {
            self.b.async_arrow_expr_body(awaited)
        };
        self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(thunk)];
        if self.dev {
            args.push(Arg::Expr(self.b.str_expr(dev_label)));

            if !self
                .ignore_query
                .is_ignored_at_span(span_start, "await_waterfall")
            {
                let (line, col) = self.component_line_index.line_col(init_span_start);
                let loc = format!("{}:{}:{}", sanitize_location(self.filename), line, col);
                args.push(Arg::Expr(self.b.str_expr(&loc)));
            }
        }

        let async_derived = self.b.call_expr("$.async_derived", args);
        self.b.await_expr(async_derived)
    }

    fn push_boxed_temp(
        &mut self,
        init: Expression<'a>,
        decl_kind: VariableDeclarationKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) -> Expression<'a> {
        let tmp_name = self.ident_gen.generate("$$d");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);
        out.push(
            self.b.ast.variable_declarator(
                SPAN,
                decl_kind,
                self.b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
                NONE,
                Some(init),
                false,
            ),
        );
        self.b.call_expr("$.get", [Arg::Ident(tmp_name_str)])
    }

    fn derived_source_is_whole_props(&self, call: &CallExpression<'a>) -> bool {
        let Some(Expression::Identifier(id)) = call
            .arguments
            .first()
            .and_then(|arg| arg.as_expression())
            .map(|expr| expr.get_inner_expression())
        else {
            return false;
        };
        let Some(sym) = self.component_scoping.symbol_for_identifier_reference(id) else {
            return false;
        };
        self.binding_semantics_for_symbol(sym)
            .is_some_and(|sem| sem.is_rest_props())
    }

    fn derived_leaf_value(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
    ) -> (&'a str, Expression<'a>) {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let value = self.wrap_derived_value(accessor);
        let value = if self.dev {
            self.b
                .call_expr("$.tag", [Arg::Expr(value), Arg::Str(name.to_string())])
        } else {
            value
        };
        (name, value)
    }

    fn derived_leaf_declarator(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
        decl_kind: VariableDeclarationKind,
    ) -> VariableDeclarator<'a> {
        let (name, value) = self.derived_leaf_value(symbol, accessor);
        self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(value),
            false,
        )
    }
}

pub(super) fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::CallExpression(call) = &expr
        && call.arguments.is_empty()
        && let Expression::Identifier(_) = &call.callee
    {
        return b.clone_expr(&call.callee);
    }
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}

fn derived_source_is_reference(source: &Expression<'_>) -> bool {
    match source.get_inner_expression() {
        Expression::Identifier(_) => true,
        Expression::StaticMemberExpression(member) => is_props_object(&member.object),
        Expression::ComputedMemberExpression(member) => is_props_object(&member.object),
        _ => false,
    }
}

fn is_props_object(object: &Expression<'_>) -> bool {
    matches!(
        object.get_inner_expression(),
        Expression::Identifier(id) if id.name == "$$props"
    )
}
