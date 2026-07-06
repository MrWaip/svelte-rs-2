import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let a = 1;
	let b = 2;
	$.store_get($$store_subs ??= {}, "$inspect", inspect)(a, b);
	$$renderer.push(`<!---->12`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
