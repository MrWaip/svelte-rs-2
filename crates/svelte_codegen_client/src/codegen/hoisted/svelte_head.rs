use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_hoisted_svelte_head(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let key = FragmentKey::SvelteHeadBody(id);
        let inner_ctx = ctx.child_of_svelte_head(key);
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body);
        let hash_str = head_hash(self.ctx.state.filename);
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.head", [Arg::Str(hash_str), Arg::Expr(body_fn)]),
        );
        Ok(())
    }
}

fn head_hash(s: &str) -> String {
    let mut h: u32 = 5381;
    for &b in s.as_bytes().iter().rev() {
        if b == b'\r' {
            continue;
        }
        h = (h.wrapping_shl(5).wrapping_sub(h)) ^ (b as u32);
    }
    to_base36(h)
}

fn to_base36(mut n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut result = Vec::new();
    while n > 0 {
        result.push(CHARS[(n % 36) as usize]);
        n /= 36;
    }
    result.reverse();
    String::from_utf8(result).unwrap_or_default()
}
