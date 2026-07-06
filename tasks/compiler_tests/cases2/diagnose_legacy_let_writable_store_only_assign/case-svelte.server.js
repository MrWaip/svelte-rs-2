import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let value = writable({ foo: 1 });
		function reset() {
			$.store_set(value, { foo: 0 });
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$value", value).foo)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
