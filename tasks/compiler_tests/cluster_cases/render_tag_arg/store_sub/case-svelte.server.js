import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { children } = $$props;
		const s = writable(0);
		children($$renderer, $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<!---->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
