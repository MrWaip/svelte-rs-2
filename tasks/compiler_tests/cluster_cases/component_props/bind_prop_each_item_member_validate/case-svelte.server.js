import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { rows = void 0 } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(rows);
			for (let i = 0, $$length = each_array.length; i < $$length; i++) {
				let row = each_array[i];
				Child($$renderer, {
					get value() {
						return row.name;
					},
					set value($$value) {
						row.name = $$value;
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
		$.bind_props($$props, { rows });
	});
}
