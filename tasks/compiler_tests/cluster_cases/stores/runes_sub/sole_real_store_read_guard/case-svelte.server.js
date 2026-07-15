import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable(0);
		const doubled = $.derived(() => $.store_get($$store_subs ??= {}, "$source", source));
		$$renderer.push(`<p>${$.escape(doubled())}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
