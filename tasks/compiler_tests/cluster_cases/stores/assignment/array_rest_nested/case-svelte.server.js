import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a, b, c;
		const s = writable([
			1,
			2,
			3
		]);
		$: [a, ...[b, c]] = $.store_get($$store_subs ??= {}, "$s", s);
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
