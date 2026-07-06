import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let total = writable(0);
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$total", total))}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
