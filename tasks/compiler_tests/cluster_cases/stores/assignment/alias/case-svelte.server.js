import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let x, y;
		const s = writable({
			a: 1,
			b: 2
		});
		$: ({a: x, b: y} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
