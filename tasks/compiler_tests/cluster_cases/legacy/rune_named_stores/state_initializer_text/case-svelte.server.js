import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let snap = $.store_get($$store_subs ??= {}, "$state", state)(5);
	$$renderer.push(`<p>snap=${$.escape(snap)}</p>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
