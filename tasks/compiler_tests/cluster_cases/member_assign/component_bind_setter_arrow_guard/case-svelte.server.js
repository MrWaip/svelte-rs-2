import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	let v = 0;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		var bind_get = () => v;
		var bind_set = (n) => obj.x = src;
		Child($$renderer, {
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
}
