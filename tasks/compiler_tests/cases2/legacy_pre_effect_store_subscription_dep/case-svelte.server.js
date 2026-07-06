import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let actions;
		function pick(value) {
			return value;
		}
		const source = writable(0);
		$: actions = pick($.store_get($$store_subs ??= {}, "$source", source));
		$$renderer.push(`<!---->${$.escape(actions)}`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
