import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let inner;
		const s = writable({ outer: [{ inner: 1 }] });
		$: ({outer: [{inner}]} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(inner)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
