import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const obj = writable({ a: 1 });
		$$renderer.push(`<input type="number"${$.attr("value", $.store_get($$store_subs ??= {}, "$obj", obj).a)}/>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
