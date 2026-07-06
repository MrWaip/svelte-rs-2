import * as $ from "svelte/internal/server";
import { state, derived } from "./store.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a = $.store_get($$store_subs ??= {}, "$state", state)(0);
		let b = $.store_get($$store_subs ??= {}, "$derived", derived)(0);
		$$renderer.push(`<p>${$.escape(a)} ${$.escape(b)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
