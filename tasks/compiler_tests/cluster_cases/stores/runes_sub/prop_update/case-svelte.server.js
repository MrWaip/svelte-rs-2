import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let { store } = $$props;
	$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$store", store))}</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
