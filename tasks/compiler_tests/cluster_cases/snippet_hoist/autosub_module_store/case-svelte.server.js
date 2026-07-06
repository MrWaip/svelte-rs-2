import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
const count = writable(0);
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function foo($$renderer) {
			$$renderer.push(`<!---->${$.escape($.store_get($$store_subs ??= {}, "$count", count))}`);
		}
		foo($$renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
