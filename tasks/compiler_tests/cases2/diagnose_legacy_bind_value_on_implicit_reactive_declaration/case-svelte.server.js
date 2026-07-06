import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let value;
		let store = $$props["store"];
		$: value = $.store_get($$store_subs ??= {}, "$store", store);
		$: if (value !== $.store_get($$store_subs ??= {}, "$store", store)) store.set(value);
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			Child($$renderer, {
				get value() {
					return value;
				},
				set value($$value) {
					value = $$value;
					$$settled = false;
				}
			});
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { store });
	});
}
