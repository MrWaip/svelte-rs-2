import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const s = writable(1);
		function run() {
			$.store_set(s, 5);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$s", s))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
