import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let inputRef = void 0;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get ref() {
				return inputRef;
			},
			set ref($$value) {
				inputRef = $$value;
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
