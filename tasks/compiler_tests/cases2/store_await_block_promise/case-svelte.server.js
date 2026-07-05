import * as $ from "svelte/internal/server";
import { promise } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$.await($$renderer, $.store_get($$store_subs ??= {}, "$promise", promise), () => {
		$$renderer.push(`<p>p</p>`);
	}, (value) => {
		$$renderer.push(`<p>${$.escape(value)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
