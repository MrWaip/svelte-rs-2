import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let { limitAmount } = $$props;
	$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$limitAmount", limitAmount))}/> <button>reset</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
