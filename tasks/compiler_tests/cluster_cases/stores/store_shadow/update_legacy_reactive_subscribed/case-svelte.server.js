import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let base = $.fallback($$props["base"], 0);
	let count = 0;
	$: count = base * 2;
	$: $.store_set(count, 1);
	$$renderer.push(`<button>increment</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { base });
}
