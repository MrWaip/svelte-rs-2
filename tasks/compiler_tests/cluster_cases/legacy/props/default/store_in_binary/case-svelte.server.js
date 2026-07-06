import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let foo = writable(42);
		let x = $.fallback($$props["x"], () => $.store_get($$store_subs ??= {}, "$foo", foo) + 1, true);
		$$renderer.push(`<p>${$.escape(x)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { x });
	});
}
