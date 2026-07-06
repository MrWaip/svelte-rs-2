import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	let value = $.fallback($$props["value"], "");
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Inner($$renderer, {
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
	$.bind_props($$props, { value });
}
