import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let { store } = $$props;
	const data = $.derived(() => store.data);
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get value() {
				return $.store_get($$store_subs ??= {}, "$data", data()).amount;
			},
			set value($$value) {
				$.store_mutate($$store_subs ??= {}, "$data", data, $.store_get($$store_subs ??= {}, "$data", data()).amount = $$value);
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
}
