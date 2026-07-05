import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a, b;
		const s = writable({
			p: { a: 1 },
			q: { b: 2 }
		});
		$: ({p: {a}, q: {b}} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
