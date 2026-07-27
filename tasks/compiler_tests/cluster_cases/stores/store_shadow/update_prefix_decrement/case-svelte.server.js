import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let count = 0;
	$: $.store_set(count, 1);
	$$renderer.push(`<button>decrement</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
