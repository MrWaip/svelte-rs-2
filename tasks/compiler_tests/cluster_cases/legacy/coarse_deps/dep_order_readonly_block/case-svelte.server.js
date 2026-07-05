import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import { foo } from "lib";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const obj = writable({});
		let c = "";
		$: {
			c;
			foo;
			$.store_get($$store_subs ??= {}, "$obj", obj);
		}
		$$renderer.push(`<input${$.attr("value", c)}/>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
