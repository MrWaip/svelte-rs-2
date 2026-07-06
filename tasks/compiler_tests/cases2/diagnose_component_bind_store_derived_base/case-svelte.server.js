import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	var $$store_subs;
	function makeStore() {
		return { search: null };
	}
	let outer = void 0;
	const $$d = $.derived(() => makeStore(outer)), search = $.derived(() => $$d().search);
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get value() {
				return $.store_get($$store_subs ??= {}, "$search", search());
			},
			set value($$value) {
				$.store_set(search, $$value);
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
