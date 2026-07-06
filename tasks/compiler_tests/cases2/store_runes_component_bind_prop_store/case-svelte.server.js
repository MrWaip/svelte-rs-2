import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let { value } = $$props;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get cents() {
				return $.store_get($$store_subs ??= {}, "$value", value);
			},
			set cents($$value) {
				$.store_set(value, $$value);
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
