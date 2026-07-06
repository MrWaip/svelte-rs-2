import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let state = "hello";
	function update() {
		state = state + "!";
	}
	$$renderer.push(`<button>update</button> `);
	if (state) {
		$$renderer.push("<!--[0-->");
		const len = state.length;
		$$renderer.push(`<span>${$.escape(len)} / ${$.escape($.store_get($$store_subs ??= {}, "$state", state))}</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
