import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let count = 0;
		$$renderer.push(`<button>increment</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
