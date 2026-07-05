import * as $ from "svelte/internal/server";
import { state } from "./store.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a = $.store_get($$store_subs ??= {}, "$state", state)(0);
		$$renderer.push(`<p>${$.escape(a)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
