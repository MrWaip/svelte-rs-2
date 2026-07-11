import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable(0);
		const held = $.derived(() => source);
		const doubled = $.derived(() => $.store_get($$store_subs ??= {}, "$held", held()));
		$$renderer.push(`<p>${$.escape(doubled())}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
