import * as $ from "svelte/internal/server";
import { store } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		$.store_mutate($$store_subs ??= {}, "$store", store, $.store_get($$store_subs ??= {}, "$store", store).field = "hello");
		$$renderer.push(`<button>set</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
