import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const state = writable({ x: 0 });
		let y = 0;
		$: y = $.store_get($$store_subs ??= {}, "$state", state).x;
		$$renderer.push(`<!---->${$.escape(y)}`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
