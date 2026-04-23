use oxc_ast::ast::Statement;
use svelte_ast::{NodeId, Span};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, Result};
use super::dispatch::PropOrSpread;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_component_prop_string(
        &self,
        name: &str,
        value_span: Span,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let value_text = self.ctx.query.component.source_text(value_span);
        let key = self.ctx.b.alloc_str(name);
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            key,
            self.ctx.b.str_expr(value_text),
        )));
    }

    pub(super) fn emit_component_prop_boolean(
        &self,
        name: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            key,
            self.ctx.b.bool_expr(true),
        )));
    }

    pub(super) fn emit_component_prop_expression(
        &mut self,
        name: &str,
        attr_id: NodeId,
        expr_id: oxc_syntax::node::NodeId,
        shorthand: bool,
        needs_memo: bool,
        is_dynamic: bool,
        items: &mut Vec<PropOrSpread<'a>>,
        memo_decls: &mut Vec<Statement<'a>>,
        memo_counter: &mut u32,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return crate::codegen::CodegenError::missing_expression(attr_id);
        };
        let expr = self.maybe_wrap_legacy_slots_read(expr);
        if needs_memo {
            let memo_name = format!("${memo_counter}");
            *memo_counter += 1;
            let thunk = self.ctx.b.thunk(expr);
            let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            memo_decls.push(self.ctx.b.let_init_stmt(&memo_name, derived));
            let memo_ref = self.ctx.b.alloc_str(&memo_name);
            let get = self.ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
            items.push(PropOrSpread::Prop(ObjProp::Getter(key, get)));
        } else if is_dynamic {
            items.push(PropOrSpread::Prop(ObjProp::Getter(key, expr)));
        } else if shorthand {
            items.push(PropOrSpread::Prop(ObjProp::Shorthand(key)));
        } else {
            items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, expr)));
        }
        Ok(())
    }

    pub(super) fn emit_component_prop_concat(
        &mut self,
        name: &str,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
        is_dynamic: bool,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let key = self.ctx.b.alloc_str(name);
        let val = self.build_concat_expr_collapse_single(attr_id, parts)?;
        if is_dynamic {
            items.push(PropOrSpread::Prop(ObjProp::Getter(key, val)));
        } else {
            items.push(PropOrSpread::Prop(ObjProp::KeyValue(key, val)));
        }
        Ok(())
    }
}
