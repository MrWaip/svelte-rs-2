import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import { count } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let state = writable(0);
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</p> <p>${$.escape($.store_get($$store_subs ??= {}, "$state", state))}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
