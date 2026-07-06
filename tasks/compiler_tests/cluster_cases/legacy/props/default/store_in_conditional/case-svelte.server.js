import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let foo = writable(42);
		let cond = true;
		let x = $.fallback($$props["x"], () => cond ? $.store_get($$store_subs ??= {}, "$foo", foo) : 0, true);
		$$renderer.push(`<p>${$.escape(x)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { x });
	});
}
