import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		class Counter {
			value = $.store_get($$store_subs ??= {}, "$state", state)(0);
		}
		let c = new Counter();
		$$renderer.push(`<button>${$.escape(c.value)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
