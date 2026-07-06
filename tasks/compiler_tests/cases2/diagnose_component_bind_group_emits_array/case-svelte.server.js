import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let value = "a";
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get group() {
				return value;
			},
			set group($$value) {
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
}
