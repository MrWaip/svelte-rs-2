use oxc_ast::ast::Expression;
use svelte_ast::{Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn debug_tag(&mut self, id: NodeId) -> Result<()> {
        let Node::DebugTag(tag) = self.component.store.get(id) else {
            return Err(CodegenError::Unsupported(id, "debug tag"));
        };
        let identifier_refs = tag.identifier_refs.clone();

        let mut props: Vec<ObjProp<'a>> = Vec::with_capacity(identifier_refs.len());
        for ident_ref in identifier_refs.iter() {
            let name: &'a str = self.b.alloc_str(self.component.source_text(ident_ref.span));
            let expr = self.take_expression(id, ident_ref)?;
            let is_shorthand =
                matches!(&expr, Expression::Identifier(ident) if ident.name.as_str() == name);
            let prop = if is_shorthand {
                ObjProp::Shorthand(name)
            } else {
                ObjProp::KeyValue(name, expr)
            };
            props.push(prop);
        }

        let obj = self.b.object_expr(props);
        let log = self.b.call_stmt("console.log", [Arg::Expr(obj)]);
        self.push_stmt(log);
        let debugger = self.b.debugger_stmt();
        self.push_stmt(debugger);
        Ok(())
    }
}
