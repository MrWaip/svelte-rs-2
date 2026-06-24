use std::collections::HashMap;
use std::iter;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{BindingPattern, Expression, VariableDeclarationKind, VariableDeclarator};
use oxc_span::SPAN;

use svelte_analyze::BindingSemantics;
use svelte_ast_builder::Arg;
use svelte_component_semantics::walk_bindings;

use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_legacy_state(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            self.rewrite_single_identifier_legacy_state(declarator, out);
            return;
        }

        let init = declarator
            .init
            .take()
            .expect("legacy state destructure declarator carries an initializer");

        let tmp_name = self.ident_gen.generate("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let root = self.b.rid_expr(tmp_name_str);
            let expr = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_declarators,
                None,
                decl_kind,
            );

            let is_reactive = self
                .analysis
                .is_some_and(|a| a.binding_semantics(v.symbol).is_legacy_state());
            let value = if is_reactive {
                self.b.call_expr("$.mutable_source", [Arg::Expr(expr)])
            } else {
                expr
            };

            let name: &'a str = self
                .b
                .alloc_str(self.component_scoping.symbol_name(v.symbol));
            leaf_declarators.push(
                self.b.ast.variable_declarator(
                    SPAN,
                    decl_kind,
                    self.b
                        .ast
                        .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
                    NONE,
                    Some(value),
                    false,
                ),
            );
        });

        out.push(self.build_tmp_declarator(decl_kind, tmp_name_str, init));
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
    }

    pub(super) fn rewrite_single_identifier_legacy_state(
        &mut self,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let immutable = match &declarator.id {
            BindingPattern::BindingIdentifier(binding) => binding
                .symbol_id
                .get()
                .and_then(|sym| self.analysis.map(|a| a.binding_semantics(sym)))
                .and_then(|sem| match sem {
                    BindingSemantics::LegacyState(state) => Some(state.immutable),
                    _ => None,
                }),
            _ => None,
        };

        let Some(immutable) = immutable else {
            out.push(declarator);
            return;
        };

        if let Some(init) = declarator.init.as_mut() {
            let init_expr = self.b.move_expr(init);
            let call = if immutable {
                self.b
                    .call_expr("$.mutable_source", [Arg::Expr(init_expr), Arg::Bool(true)])
            } else {
                self.b.call_expr("$.mutable_source", [Arg::Expr(init_expr)])
            };
            declarator.init = Some(call);
        } else {
            let call = if immutable {
                self.b.call_expr(
                    "$.mutable_source",
                    [Arg::Expr(self.b.void_zero_expr()), Arg::Bool(true)],
                )
            } else {
                self.b.call_expr("$.mutable_source", iter::empty::<Arg>())
            };
            declarator.init = Some(call);
        }
        out.push(declarator);
    }

    fn build_tmp_declarator(
        &self,
        decl_kind: VariableDeclarationKind,
        name: &'a str,
        value: Expression<'a>,
    ) -> VariableDeclarator<'a> {
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
