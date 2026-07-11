import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var $$store_subs;
	let store = undefined;
	let value = $.derived(() => $.store_get($$store_subs ??= {}, "$store", store));
	$$renderer.push(`<h1>${$.escape(value())}</h1>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
