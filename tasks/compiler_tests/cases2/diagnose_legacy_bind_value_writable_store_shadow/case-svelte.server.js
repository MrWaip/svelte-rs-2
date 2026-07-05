import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let x;
		let value = writable("");
		$: x = $.store_get($$store_subs ??= {}, "$value", value);
		$$renderer.push(`<input${$.attr("value", value)}/> <p>${$.escape(x)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
