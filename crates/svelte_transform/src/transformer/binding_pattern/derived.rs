use std::collections::HashMap;
use std::iter;
use std::mem;

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, CallExpression, Expression, Statement, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_span::SPAN;

use svelte_analyze::{
    BindingSemantics, DerivedKind, PropBindingKind, PropBindingSemantics, RuneKind,
};
use svelte_ast_builder::{Arg, AssignLeft};
use svelte_component_semantics::{SymbolId, walk_bindings};

use super::super::location::sanitize_location;
use super::super::model::{AsyncDerivedMode, ComponentTransformer};

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_derived(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        derived_kind: DerivedKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
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
                self.b.thunk(arg)
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
        let tmp_init = self.async_derived_init(init, &declarator.id, span_start);
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
            self.b.block_stmt(block)
        } else {
            let mut decls: OxcVec<'a, VariableDeclarator<'a>> = self.b.ast.vec();
            decls.push(self.b.ast.variable_declarator(
                SPAN,
                decl_kind,
                self.b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
                NONE,
                Some(tmp_init),
                false,
            ));
            for carrier in carrier_declarators {
                decls.push(carrier);
            }
            for (symbol, access) in leaves {
                decls.push(self.derived_leaf_declarator(symbol, access, decl_kind));
            }
            let decl = self.b.ast.variable_declaration(SPAN, decl_kind, decls, false);
            Statement::VariableDeclaration(self.b.alloc(decl))
        }
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
        walk_bindings(pattern, |v| {
            let root = root_template.clone_in(self.b.ast.allocator);
            let access = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                carriers,
                carrier_declarators,
                None,
                decl_kind,
            );
            leaves.push((v.symbol, access));
        });
        leaves
    }

    fn async_derived_init(
        &mut self,
        init: Expression<'a>,
        pattern: &BindingPattern<'a>,
        span_start: u32,
    ) -> Expression<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("async $derived destructure initializer is a call");
        };
        let init_span_start = call.span.start;
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        let awaited = dummy.into_expression();

        let thunk = if let Expression::AwaitExpression(await_expr) = awaited {
            let source_expr = await_expr.unbox().argument;
            let await_inner = self.b.await_expr(source_expr);
            self.b.async_thunk(await_inner)
        } else {
            self.b.async_arrow_expr_body(awaited)
        };

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(thunk)];
        if self.dev {
            let kind = match pattern {
                BindingPattern::ArrayPattern(_) => "iterable",
                _ => "object",
            };
            args.push(Arg::Expr(self.b.str_expr(&format!("[$derived {kind}]"))));

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
        match self.async_derived_mode() {
            AsyncDerivedMode::Await => self.b.await_expr(async_derived),
            AsyncDerivedMode::Save => {
                let saved = self.b.call_expr("$.save", [Arg::Expr(async_derived)]);
                self.b
                    .call_expr_callee(self.b.await_expr(saved), iter::empty::<Arg<'a, '_>>())
            }
        }
    }

    fn push_boxed_temp(
        &mut self,
        init: Expression<'a>,
        decl_kind: VariableDeclarationKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) -> Expression<'a> {
        let tmp_name = self.ident_gen.generate("$$d");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);
        out.push(self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
            NONE,
            Some(init),
            false,
        ));
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
        matches!(
            self.binding_semantics_for_symbol(sym),
            Some(BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::Rest,
                ..
            }))
        )
    }

    fn derived_leaf_value(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
    ) -> (&'a str, Expression<'a>) {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let value = self.wrap_state_value(accessor, RuneKind::Derived, false);
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
