import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const count = writable(0);
		function foo($$renderer) {
			$$renderer.push(`<!---->${$.escape($.store_get($$store_subs ??= {}, "$count", count))}`);
		}
		foo($$renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
