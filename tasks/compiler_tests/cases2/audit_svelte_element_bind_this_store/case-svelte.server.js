import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let tag = "div";
		const store = writable();
		$.element($$renderer, tag, void 0, () => {
			$$renderer.push(`x`);
		});
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
