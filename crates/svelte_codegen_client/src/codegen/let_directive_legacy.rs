use oxc_ast::ast::{BindingPattern, Expression, Statement, VariableDeclarationKind};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_ast::{Attribute, Node, NodeId};
use svelte_ast_builder::Arg;

use super::Codegen;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_let_directive_legacy_stmts(
        &mut self,
        owner_id: NodeId,
    ) -> Vec<Statement<'a>> {
        let node = self.ctx.query.component.store.get(owner_id);
        let attrs: &[Attribute] = match node {
            Node::Element(el) => &el.attributes,
            Node::SlotElementLegacy(el) => &el.attributes,
            Node::SvelteFragmentLegacy(el) => &el.attributes,
            _ => match node.as_component_like() {
                Some(view) => view.attributes,
                None => return Vec::new(),
            },
        };
        let let_dirs: Vec<svelte_ast::LetDirectiveLegacy> = attrs
            .iter()
            .filter_map(|a| match a {
                Attribute::LetDirectiveLegacy(d) => Some(d.clone()),
                _ => None,
            })
            .collect();
        let mut out: Vec<Statement<'a>> = Vec::new();
        for dir in &let_dirs {
            if let Some(stmt) = self.build_let_directive_legacy_stmt(dir) {
                out.push(stmt);
            }
        }
        out
    }

    pub(in crate::codegen) fn default_slot_has_let_directive_legacy(
        &self,
        component_id: NodeId,
    ) -> bool {
        let store = &self.ctx.query.component.store;
        let Some(view) = store.get(component_id).as_component_like() else {
            return false;
        };
        if view
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::LetDirectiveLegacy(_)))
        {
            return true;
        }
        store
            .fragment_nodes(view.fragment)
            .iter()
            .any(|&child_id| match store.get(child_id) {
                Node::SvelteFragmentLegacy(el) => el
                    .attributes
                    .iter()
                    .any(|a| matches!(a, Attribute::LetDirectiveLegacy(_))),
                _ => false,
            })
    }

    fn build_let_directive_legacy_stmt(
        &mut self,
        dir: &svelte_ast::LetDirectiveLegacy,
    ) -> Option<Statement<'a>> {
        use oxc_allocator::CloneIn;
        let binding_ref = dir.binding.as_ref()?;
        let stmt_id = binding_ref.id();

        let (is_destructured, simple_name, simple_init_clone): (
            bool,
            Option<String>,
            Option<Expression<'a>>,
        ) = {
            let stmt_ref = self.ctx.state.parsed.stmt(stmt_id)?;
            let Statement::VariableDeclaration(decl) = stmt_ref else {
                return None;
            };
            let declarator = decl.declarations.first()?;
            match &declarator.id {
                BindingPattern::BindingIdentifier(id) => {
                    let name = id.name.as_str().to_string();
                    let init = declarator.init.as_ref()?;
                    let init_clone = init.clone_in(self.ctx.b.ast.allocator);
                    (false, Some(name), Some(init_clone))
                }
                BindingPattern::ObjectPattern(_)
                | BindingPattern::ArrayPattern(_) => (true, None, None),
                _ => return None,
            }
        };

        if !is_destructured {
            let name = simple_name?;
            let init_expr = simple_init_clone?;
            let thunk = self.ctx.b.thunk(init_expr);
            let derived_fn = self.ctx.query.view.derived_helper();
            let derived = self.ctx.b.call_expr(derived_fn, [Arg::Expr(thunk)]);
            return Some(self.ctx.b.const_stmt(&name, derived));
        }

        self.build_destructured_let_directive_legacy_stmt(stmt_id)
    }

    fn build_destructured_let_directive_legacy_stmt(
        &mut self,
        stmt_id: OxcNodeId,
    ) -> Option<Statement<'a>> {
        use oxc_allocator::CloneIn;
        let (stmt_clone, stmt_oxc_node_id, binding_names) = {
            let stmt_ref = self.ctx.state.parsed.stmt(stmt_id)?;
            let Statement::VariableDeclaration(decl) = stmt_ref else {
                return None;
            };
            let declarator = decl.declarations.first()?;
            let stmt_oxc_node_id = decl.node_id();

            let mut binding_names: Vec<String> = Vec::new();
            svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                binding_names.push(self.ctx.symbol_name(v.symbol).to_string());
            });

            let stmt_clone = stmt_ref.clone_in(self.ctx.b.ast.allocator);
            (stmt_clone, stmt_oxc_node_id, binding_names)
        };

        let carrier_sym_id = match self.ctx.query.view.declarator_semantics(stmt_oxc_node_id) {
            svelte_analyze::DeclaratorSemantics::LetCarrier { carrier_symbol } => carrier_symbol,
            _ => return None,
        };
        let carrier_name = self.ctx.symbol_name(carrier_sym_id).to_string();

        let mut destructure_stmt = stmt_clone;
        if let Statement::VariableDeclaration(d) = &mut destructure_stmt {
            d.kind = VariableDeclarationKind::Let;
        }

        let mut body = vec![destructure_stmt];
        body.push(
            self.ctx
                .b
                .return_stmt(self.ctx.b.shorthand_object_expr(&binding_names)),
        );
        let derived = self
            .ctx
            .b
            .call_expr("$.derived", [Arg::Expr(self.ctx.b.thunk_block(body))]);
        Some(self.ctx.b.const_stmt(&carrier_name, derived))
    }
}
