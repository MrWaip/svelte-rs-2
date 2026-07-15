import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		const obj = {};
		function run() {
			$.store_set(a, $.fallback(obj.$a, 5));
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$a", a))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
