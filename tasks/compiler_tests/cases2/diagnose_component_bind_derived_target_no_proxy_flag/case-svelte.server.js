import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let base = false;
	let derivedFlag = $.derived(() => base);
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get value() {
				return derivedFlag();
			},
			set value($$value) {
				derivedFlag($$value);
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
