use analyze_hir::OwnerContentTypeFlags;
use ast_builder::{BuilderExpression as BExpr, BuilderFunctionArgument as BArg};
use hir::{NodeId, OwnerId};
use oxc_ast::ast::Statement;

use crate::context::{FragmentContext, OwnerContext};

use super::{
    interpolation::TransformInterpolationOptions, template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_fragment(
        &mut self,
        nodes: &Vec<NodeId>,
        self_owner_id: OwnerId,
        content_type: OwnerContentTypeFlags,
    ) -> Vec<Statement<'hir>> {
        if content_type.is_empty() {
            return Vec::new();
        }

        if content_type.only_text() {
            return self.fragment_text_shortcut(self_owner_id, nodes);
        }

        if content_type.any_interpolation_like() {
            return self.fragment_interpolation_shortcut(self_owner_id, nodes);
        }

        if nodes.len() == 1
            && self
                .store
                .lookup_node(nodes[0], |node| node.is_elseif_block())
        {
            return self.fragment_without_template_shortcut(self_owner_id, nodes);
        }

        if content_type.only_element() && nodes.len() == 1 {
            return self.fragment_element_shortcut(self_owner_id, nodes);
        }

        if content_type.only_synthetic_node() && nodes.len() == 1 {
            return self.fragment_synthetic_shortcut(self_owner_id, nodes);
        }

        self.fragment_common(self_owner_id, nodes)
    }

    fn fragment_common(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        let mut body = Vec::new();
        let mut context = FragmentContext::new();
        let template_name = self.analyses.generate_ident("root");

        let identifier = self.analyses.generate_ident("fragment");
        let anchor = self
            .b
            .call_expr("$.first_child", [BArg::Ident(&identifier)]);
        let owner_ctx = OwnerContext::new(&mut context, anchor, self.b, self_owner_id);

        self.handle_first_text_like(self_owner_id, &mut body);

        self.transform_nodes(nodes, owner_ctx);
        self.add_template(&mut context, &template_name, Some(1.0));

        body.push(
            self.b
                .var(&identifier, BExpr::Call(self.b.call(&template_name, []))),
        );
        self.build_fragment(context, &mut body);

        let close = self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        );

        body.push(close);

        body
    }

    fn build_template_effect(&self, update: Vec<Statement<'hir>>) -> Statement<'hir> {
        let call = self.b.call_stmt(
            "$.template_effect",
            [BArg::Arrow(self.b.arrow(self.b.params([]), update))],
        );

        call
    }

    fn fragment_without_template_shortcut(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        // to match numbers of ident with svelte
        self.analyses.generate_ident("root");

        let mut body = Vec::new();
        let mut fragment_ctx = FragmentContext::new();
        let node = self.store.get_node(nodes[0]);

        let anchor = self.b.cheap_expr();
        let mut owner_ctx = OwnerContext::new(&mut fragment_ctx, anchor, self.b, self_owner_id);

        self.transform_node(node, &mut owner_ctx);

        self.build_fragment(fragment_ctx, &mut body);

        body
    }

    fn fragment_synthetic_shortcut(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        // to match numbers of ident with svelte
        self.analyses.generate_ident("root");

        let mut body = Vec::new();

        let identifier = self.analyses.generate_ident("fragment");
        let anchor = self
            .b
            .call_expr("$.first_child", [BArg::Ident(&identifier)]);

        let mut fragment_ctx = FragmentContext::new();
        let owner_ctx = OwnerContext::new(&mut fragment_ctx, anchor, self.b, self_owner_id);

        self.transform_nodes(nodes, owner_ctx);

        body.push(
            self.b
                .var(&identifier, BExpr::Call(self.b.call("$.comment", []))),
        );

        self.build_fragment(fragment_ctx, &mut body);

        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        body
    }

    /// Build a fragment for interpolation or concatenation
    ///
    /// !svelte specific optimization
    fn fragment_interpolation_shortcut(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        let node = self.store.get_node(nodes[0]);
        let identifier = self.analyses.generate_ident("text");
        let anchor = self.b.rid_expr(&identifier);
        let mut body: Vec<Statement<'hir>> = vec![];

        let mut fragment_ctx = FragmentContext::new();
        let mut owner_ctx = OwnerContext::new(&mut fragment_ctx, anchor, self.b, self_owner_id);

        match node {
            hir::Node::Interpolation(interpolation) => {
                self.transform_interpolation(
                    interpolation,
                    &mut owner_ctx,
                    TransformInterpolationOptions::default(),
                );
            }

            hir::Node::Concatenation(concatenation) => {
                self.transform_concatenation(
                    concatenation,
                    &mut owner_ctx,
                    TransformInterpolationOptions::default(),
                );
            }
            _ => unreachable!(),
        };

        self.handle_first_text_like(self_owner_id, &mut body);
        let call = self.b.call("$.text", []);
        body.push(self.b.var(&identifier, BExpr::Call(call)));
        self.build_fragment(fragment_ctx, &mut body);

        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        body
    }

    /// Builds a fragment that contains only one text node
    ///
    /// !svelte specific optimization
    fn fragment_text_shortcut(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        let identifier = self.analyses.generate_ident("text");
        let mut body = Vec::new();

        let text = self.store.get_node(nodes[0]).as_text().unwrap();
        let call = self.b.call("$.text", [BArg::Str(text.value.to_string())]);

        self.handle_first_text_like(self_owner_id, &mut body);
        body.push(self.b.var(&identifier, BExpr::Call(call)));
        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        body
    }

    /// Builds a fragment that contains only one text node
    ///
    /// !svelte specific optimization
    fn fragment_element_shortcut(
        &mut self,
        self_owner_id: OwnerId,
        nodes: &Vec<NodeId>,
    ) -> Vec<Statement<'hir>> {
        let template_name = self.analyses.generate_ident("root");
        let mut body = Vec::new();
        let element = self.store.get_node(nodes[0]).as_element().unwrap();
        let identifier = self.analyses.generate_ident(element.name);

        let mut fragment_ctx = FragmentContext::new();
        let mut owner_ctx = OwnerContext::new(
            &mut fragment_ctx,
            self.b.rid_expr(&identifier),
            self.b,
            self_owner_id,
        );

        self.transform_element(element, &mut owner_ctx);
        self.add_template(&mut fragment_ctx, &template_name, None);

        body.push(
            self.b
                .var(&identifier, BExpr::Call(self.b.call(&template_name, []))),
        );
        self.build_fragment(fragment_ctx, &mut body);
        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        body
    }

    fn build_fragment(
        &mut self,
        fragment_ctx: FragmentContext<'hir>,
        body: &mut Vec<Statement<'hir>>,
    ) {
        body.extend(fragment_ctx.before_init);
        body.extend(fragment_ctx.init);

        if !fragment_ctx.update.is_empty() {
            body.push(self.build_template_effect(fragment_ctx.update));
        }

        body.extend(fragment_ctx.after_update);
    }

    fn add_template(
        &mut self,
        ctx: &mut FragmentContext<'hir>,
        name: &str,
        bit_flags: Option<f64>,
    ) {
        let template = ctx.template.concat();
        let lit = self.b.template_from_str(&template);
        let mut args = vec![BArg::TemplateStr(lit)];

        if let Some(flags) = bit_flags {
            args.push(BArg::Num(flags));
        }

        let call = self.b.call("$.template", args);

        let var = self.b.var(name, BExpr::Call(call));

        self.hoisted.push(var);
    }

    fn handle_first_text_like(&mut self, self_owner_id: OwnerId, body: &mut Vec<Statement<'hir>>) {
        let owner = self.store.get_owner(self_owner_id);

        if self
            .store
            .is_first_of(self_owner_id, |node| node.is_text_like())
            && owner.is_require_next()
        {
            body.push(self.b.call_stmt("$.next", []))
        }
    }
}
