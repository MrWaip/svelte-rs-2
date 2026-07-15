import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const data = writable({ docs: [] });
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$data", data).docs);
			for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
				let doc = each_array[idx];
				Child($$renderer, {
					get document() {
						return doc;
					},
					set document($$value) {
						doc = $$value;
						$$settled = false;
					}
				});
			}
			$$renderer.push(`<!--]-->`);
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
