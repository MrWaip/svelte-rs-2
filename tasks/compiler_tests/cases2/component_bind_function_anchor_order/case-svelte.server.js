import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	let value = 0;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		var bind_get = () => value;
		var bind_set = (v) => value = v;
		$$renderer.push(`<div>`);
		Comp($$renderer, {
			get value() {
				return bind_get();
			},
			set value($$value) {
				bind_set($$value);
			}
		});
		$$renderer.push(`<!----></div>`);
	}
	do {
		$$settled = true;
		$$inner_renderer = $$renderer.copy();
		$$render_inner($$inner_renderer);
	} while (!$$settled);
	$$renderer.subsume($$inner_renderer);
}
