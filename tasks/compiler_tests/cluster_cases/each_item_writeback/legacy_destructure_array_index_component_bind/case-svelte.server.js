import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let [first] = each_array[$$index];
			Child($$renderer, {
				get value() {
					return first;
				},
				set value($$value) {
					first = $$value;
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
}
