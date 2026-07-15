import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const x = writable(1);
		const k = "a";
		const obj = { a: 42 };
		function run() {
			$.store_set(x, obj[k]);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$x", x))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
