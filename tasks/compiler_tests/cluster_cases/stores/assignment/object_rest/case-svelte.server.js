import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a, rest;
		const s = writable({
			a: 1,
			b: 2,
			c: 3
		});
		$: ({a, ...rest} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.b)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
