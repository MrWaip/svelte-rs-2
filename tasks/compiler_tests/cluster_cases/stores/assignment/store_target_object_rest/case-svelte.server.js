import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		let rest;
		const obj = {
			$a: 1,
			c: 2,
			d: 3
		};
		function run() {
			$.store_set(a, obj.$a), rest = $.exclude_from_object(obj, ["$a"]);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$a", a))}${$.escape(rest.c)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
