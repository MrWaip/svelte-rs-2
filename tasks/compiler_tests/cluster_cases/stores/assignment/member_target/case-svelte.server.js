import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a;
		const s = writable([1, 2]);
		$: [globalThis.foo, a] = $.store_get($$store_subs ??= {}, "$s", s);
		$$renderer.push(`<button>${$.escape(a)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
