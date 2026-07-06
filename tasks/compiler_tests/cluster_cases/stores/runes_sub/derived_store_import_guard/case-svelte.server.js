import * as $ from "svelte/internal/server";
import { writable, derived } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const store = writable(0);
		let doubled = $.derived(() => $.store_get($$store_subs ??= {}, "$store", store) * 2);
		$$renderer.push(`<p>${$.escape(doubled())} ${$.escape($.store_get($$store_subs ??= {}, "$store", store))}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
