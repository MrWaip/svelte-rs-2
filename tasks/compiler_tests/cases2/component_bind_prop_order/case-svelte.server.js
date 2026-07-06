import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let v = void 0;
	function onx() {}
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			a: "1",
			onx,
			b: "2",
			get value() {
				return v;
			},
			set value($$value) {
				v = $$value;
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
}
