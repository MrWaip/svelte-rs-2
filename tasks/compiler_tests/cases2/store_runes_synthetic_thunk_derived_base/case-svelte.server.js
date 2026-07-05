import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { manager } = $$props;
		let store = $.derived(() => manager.store);
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$store", store()))}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
