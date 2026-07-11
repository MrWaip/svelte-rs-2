import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const x = writable(1);
		const y = writable(2);
		function run() {
			(($$value) => {
				$.store_set(x, $$value.a);
				$.store_set(y, $$value.b);
			})({
				a: 9,
				b: 10
			});
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$x", x))}${$.escape($.store_get($$store_subs ??= {}, "$y", y))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
