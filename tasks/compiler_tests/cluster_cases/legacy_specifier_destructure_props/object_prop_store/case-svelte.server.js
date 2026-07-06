import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let tmp = {
			a: 1,
			c: writable(2)
		}, a = $.fallback($$props["a"], () => tmp.a, true), c = $.fallback($$props["c"], () => tmp.c, true);
		function inc() {
			a++;
		}
		$$renderer.push(`<button>${$.escape(a)}${$.escape($.store_get($$store_subs ??= {}, "$c", c))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			a,
			c
		});
	});
}
