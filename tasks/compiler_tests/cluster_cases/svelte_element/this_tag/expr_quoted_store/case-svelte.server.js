import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const tag = writable("div");
		$.element($$renderer, $.store_get($$store_subs ??= {}, "$tag", tag), void 0, () => {
			$$renderer.push(`hello`);
		});
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
