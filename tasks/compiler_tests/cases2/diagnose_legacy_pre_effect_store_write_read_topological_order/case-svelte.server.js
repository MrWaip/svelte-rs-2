import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const w = writable(0);
		let derived = 0;
		$: (() => {
			$.store_set(w, 1);
		})();
		$: derived = $.store_get($$store_subs ??= {}, "$w", w) * 2;
		$$renderer.push(`<p>${$.escape(derived)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
