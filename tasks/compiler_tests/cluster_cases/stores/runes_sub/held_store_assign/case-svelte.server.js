import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable(0);
		const held = $.derived(() => source);
		function replace() {
			$.store_set(held, writable(1));
		}
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$held", held()))}</p> <button>replace</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
