use std::collections::HashMap;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{BindingPattern, Expression, VariableDeclarationKind, VariableDeclarator};
use oxc_span::SPAN;

use svelte_analyze::BindingSemantics;
use svelte_component_semantics::walk_bindings;
use svelte_emit_builders::props::build_legacy_prop_call;

use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_legacy_props(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        if let BindingPattern::BindingIdentifier(id) = &declarator.id {
            let Some(sym) = id.symbol_id.get() else {
                out.push(declarator);
                return;
            };
            let Some(BindingSemantics::LegacyBindableProp(legacy)) =
                self.analysis.map(|a| a.binding_semantics(sym))
            else {
                out.push(declarator);
                return;
            };
            let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(sym));
            let init = declarator.init.take();
            let call =
                build_legacy_prop_call(self.b, self.gen_arrow_scope, name, None, legacy, init);
            out.push(self.build_leaf_declarator(decl_kind, name, call));
            return;
        }

        let init = declarator
            .init
            .take()
            .expect("legacy export-let destructure declarator carries an initializer");

        let tmp_name = self.ident_gen.generate("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let root = self.b.rid_expr(tmp_name_str);
            let access = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_declarators,
                None,
                decl_kind,
            );

            let name: &'a str = self
                .b
                .alloc_str(self.component_scoping.symbol_name(v.symbol));
            let value = match self.analysis.map(|a| a.binding_semantics(v.symbol)) {
                Some(BindingSemantics::LegacyBindableProp(legacy)) => build_legacy_prop_call(
                    self.b,
                    self.gen_arrow_scope,
                    name,
                    None,
                    legacy,
                    Some(access),
                ),
                _ => access,
            };
            leaf_declarators.push(self.build_leaf_declarator(decl_kind, name, value));
        });

        out.push(self.build_leaf_declarator(decl_kind, tmp_name_str, init));
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
    }

    fn build_leaf_declarator(
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
