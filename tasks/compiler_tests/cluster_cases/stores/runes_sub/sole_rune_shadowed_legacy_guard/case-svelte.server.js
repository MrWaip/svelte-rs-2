import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let state = 5;
	let foo = $.store_get($$store_subs ??= {}, "$state", state)(0);
	$$renderer.push(`<button>${$.escape(foo)} 5</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
