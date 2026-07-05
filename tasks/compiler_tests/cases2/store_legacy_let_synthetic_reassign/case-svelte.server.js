import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let count = writable(0);
		function swap() {
			count = writable(10);
		}
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
