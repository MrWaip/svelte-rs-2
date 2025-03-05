use ast_builder::{BuilderExpression as BExpr, BuilderFunctionArgument as BArg};
use hir::{NodeId, OwnerId};
use oxc_ast::ast::Statement;

use super::{
    context::{FragmentContext, OwnerContext},
    template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_fragment(
        &mut self,
        nodes: &Vec<NodeId>,
        owner_id: OwnerId,
    ) -> Vec<Statement<'hir>> {
        let content_type = self.analyses.get_common_content_type(&owner_id);

        if content_type.is_empty() {
            return Vec::new();
        }

        dbg!(content_type, content_type.only_text());

        if content_type.only_text() {
            return self.fragment_text_shortcut(owner_id);
        }

        if content_type.any_text_like() {
            return self.fragment_interpolation_shortcut(owner_id);
        }

        if content_type.only_element() && nodes.len() == 1 {
            return self.fragment_element_shortcut(owner_id);
        }

        return self.fragment_common(owner_id, nodes);
    }

    fn fragment_common(&mut self, owner_id: OwnerId, nodes: &Vec<NodeId>) -> Vec<Statement<'hir>> {
        let mut body = Vec::new();
        let mut context = FragmentContext::new();

        let identifier = "fragment";
        // let anchor = self.b.call_expr("$.first_child", [BArg::Ident(identifier)]);
        let anchor = self.b.call_expr("$.first_child", [BArg::Ident(identifier)]);
        let owner_ctx = OwnerContext::new(&mut context, anchor, self.b);

        if self.store.is_first_of(owner_id, |node| node.is_text_like()) {
            body.push(self.b.call_stmt("$.next", []));
        }

        self.transform_nodes(nodes, owner_ctx);
        self.add_template(&mut context, "root", Some(1.0));

        body.push(self.b.var(identifier, BExpr::Call(self.b.call("root", []))));
        self.build_fragment(context, &mut body);

        let close = self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        );

        body.push(close);

        return body;
    }

    fn build_template_effect(&self, update: Vec<Statement<'hir>>) -> Statement<'hir> {
        let call = self.b.call_stmt(
            "$.template_effect",
            [BArg::Arrow(self.b.arrow(self.b.params([]), update))],
        );

        return call;
    }

    /// Build a fragment for interpolation or concatenation
    ///
    /// !svelte specific optimization
    fn fragment_interpolation_shortcut(&mut self, owner_id: OwnerId) -> Vec<Statement<'hir>> {
        let node = self.store.first_of(owner_id).unwrap();
        let identifier = "text";
        let anchor = self.b.rid_expr(identifier);
        let mut body: Vec<Statement<'hir>> = vec![self.b.call_stmt("$.next", [])];

        let mut fragment_ctx = FragmentContext::new();
        let mut owner_ctx = OwnerContext::new(&mut fragment_ctx, anchor, self.b);

        match node {
            hir::Node::Interpolation(interpolation) => {
                self.transform_interpolation(interpolation, &mut owner_ctx);
            }

            hir::Node::Concatenation(concatenation) => {
                self.transform_concatenation(concatenation, &mut owner_ctx);
            }
            _ => unreachable!(),
        };

        let call = self.b.call("$.text", []);
        body.push(self.b.var(&identifier, BExpr::Call(call)));
        self.build_fragment(fragment_ctx, &mut body);

        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        return body;
    }

    /// Builds a fragment that contains only one text node
    ///
    /// !svelte specific optimization
    fn fragment_text_shortcut(&mut self, owner_id: OwnerId) -> Vec<Statement<'hir>> {
        let identifier = "text";
        let mut body = Vec::new();
        let text = self.store.first_of(owner_id).unwrap().as_text().unwrap();

        let call = self.b.call("$.text", [BArg::Str(text.value.to_string())]);

        body.push(self.b.call_stmt("$.next", []));
        body.push(self.b.var(&identifier, BExpr::Call(call)));
        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        return body;
    }

    /// Builds a fragment that contains only one text node
    ///
    /// !svelte specific optimization
    fn fragment_element_shortcut(&mut self, owner_id: OwnerId) -> Vec<Statement<'hir>> {
        let mut body = Vec::new();
        let element = self.store.first_of(owner_id).unwrap().as_element().unwrap();
        let identifier = element.name;

        let mut fragment_ctx = FragmentContext::new();
        let mut owner_ctx =
            OwnerContext::new(&mut fragment_ctx, self.b.rid_expr(identifier), self.b);

        self.transform_element(&element, &mut owner_ctx);
        self.add_template(&mut fragment_ctx, "root", None);

        body.push(self.b.var(identifier, BExpr::Call(self.b.call("root", []))));
        self.build_fragment(fragment_ctx, &mut body);
        body.push(self.b.call_stmt(
            "$.append",
            [BArg::Ident("$$anchor"), BArg::Ident(&identifier)],
        ));

        return body;
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
}
