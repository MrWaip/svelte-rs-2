import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let s = writable(0);
		function swap() {
			s = writable(1);
		}
		$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$s", s))}/> <button>swap</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
