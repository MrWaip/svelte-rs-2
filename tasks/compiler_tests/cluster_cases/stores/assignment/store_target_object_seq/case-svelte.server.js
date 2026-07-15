import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		const b = writable(2);
		const obj = {
			$a: 10,
			$b: 20
		};
		function run() {
			$.store_set(a, obj.$a), $.store_set(b, obj.$b);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$a", a))}${$.escape($.store_get($$store_subs ??= {}, "$b", b))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
