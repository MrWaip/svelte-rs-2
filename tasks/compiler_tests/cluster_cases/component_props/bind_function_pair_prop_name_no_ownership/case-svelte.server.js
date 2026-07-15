import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		var bind_get = () => value;
		var bind_set = (v) => value = v?.trim();
		Comp($$renderer, {
			get value() {
				return bind_get();
			},
			set value($$value) {
				bind_set($$value);
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
