import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const count = writable(0);
		$$renderer.push(`<div>${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</div>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
