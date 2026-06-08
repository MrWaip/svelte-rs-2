use std::collections::HashMap;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{Expression, VariableDeclarationKind, VariableDeclarator};
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

            let is_reactive = matches!(
                self.analysis.map(|a| a.binding_semantics(v.symbol)),
                Some(BindingSemantics::LegacyState(_))
            );
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
