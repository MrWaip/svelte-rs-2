import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = $$props["store"];
		function clear() {
			$.store_set(store, undefined);
		}
		function bump() {
			$.store_mutate($$store_subs ??= {}, "$store", store, $.store_get($$store_subs ??= {}, "$store", store).x = 1);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$store", store)?.x)}</button> <button>b</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { store });
	});
}
