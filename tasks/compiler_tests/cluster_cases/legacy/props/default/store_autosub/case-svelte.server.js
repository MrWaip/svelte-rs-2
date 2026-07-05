import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let foo = writable(42);
		let initial_foo = $.fallback($$props["initial_foo"], () => $.store_get($$store_subs ??= {}, "$foo", foo), true);
		$$renderer.push(`<p>${$.escape(initial_foo)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { initial_foo });
	});
}
