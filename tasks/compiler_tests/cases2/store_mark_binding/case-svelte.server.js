import * as $ from "svelte/internal/server";
import { count } from "./stores";
import Component from "./Component.svelte";
export default function App($$renderer) {
	var $$store_subs;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Component($$renderer, {
			get value() {
				return $.store_get($$store_subs ??= {}, "$count", count);
			},
			set value($$value) {
				$.store_set(count, $$value);
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
