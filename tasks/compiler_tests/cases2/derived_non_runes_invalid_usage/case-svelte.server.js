import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let source = { value: 1 };
	let { value } = $.store_get($$store_subs ??= {}, "$derived", derived)(source);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
