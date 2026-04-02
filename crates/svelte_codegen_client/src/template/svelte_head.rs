//! SvelteHead code generation — `<svelte:head>...</svelte:head>`.

use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::gen_fragment;

/// Generate `$.head(hash, ($$anchor) => { ... })`.
pub(crate) fn gen_svelte_head<'a>(ctx: &mut Ctx<'a>, id: NodeId, stmts: &mut Vec<Statement<'a>>) {
    let body = gen_fragment(ctx, FragmentKey::SvelteHeadBody(id));
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);

    // hash(filename) — Svelte uses djb2 hash of the filename for scope isolation.
    let hash_str = hash(ctx.state.filename);

    stmts.push(
        ctx.b
            .call_stmt("$.head", [Arg::Str(hash_str), Arg::Expr(body_fn)]),
    );
}

/// Svelte's hash — iterates string bytes in reverse, returns base-36 string.
/// Matches: `((hash << 5) - hash) ^ charCode`, strips `\r`.
fn hash(s: &str) -> String {
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
    String::from_utf8(result).unwrap()
}
