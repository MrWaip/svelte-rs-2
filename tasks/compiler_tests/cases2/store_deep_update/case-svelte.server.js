import * as $ from "svelte/internal/server";
import { store } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		$$renderer.push(`<button>inc</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
