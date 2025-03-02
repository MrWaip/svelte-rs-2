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

        if content_type.only_text() {
            return self.fragment_text_shortcut(owner_id);
        }

        if content_type.any_text_like() {
            return self.fragment_interpolation_shortcut(owner_id);
        }

        let mut context = FragmentContext::new();
        let mut body = Vec::new();

        let identifier = "text";

        if self.store.is_first_of(owner_id, |node| node.is_text_like()) {
            body.push(self.b.call_stmt("$.next", []));
        }

        self.transform_nodes(nodes, &mut context);
        self.add_template(&mut context, identifier, Some(1.0));

        body.extend(context.before_init);
        body.extend(context.init);

        if !context.update.is_empty() {
            body.push(self.build_template_effect(context.update));
        }

        body.extend(context.after_update);

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

    /// Build a fragment for interpolation or concatenation
    ///
    /// !svelte specific optimization
    fn fragment_interpolation_shortcut(&mut self, owner_id: OwnerId) -> Vec<Statement<'hir>> {
        let node = self.store.first_of(owner_id).unwrap();
        let identifier = "text";
        let mut body: Vec<Statement<'hir>> = vec![self.b.call_stmt("$.next", [])];

        let mut fragment_ctx = FragmentContext::new();
        let mut owner_ctx = OwnerContext::new(&mut fragment_ctx);

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

        body.extend(fragment_ctx.before_init);
        body.extend(fragment_ctx.init);

        if !fragment_ctx.update.is_empty() {
            body.push(self.build_template_effect(fragment_ctx.update));
        }

        body.extend(fragment_ctx.after_update);

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
}
