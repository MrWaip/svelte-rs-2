import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const handler = writable();
		$$renderer.push(`<button>x</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
