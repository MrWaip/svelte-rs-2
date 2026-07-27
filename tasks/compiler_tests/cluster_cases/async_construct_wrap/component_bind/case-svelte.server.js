import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		$$renderer.push(`<button>inc</button> `);
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			Child($$renderer, {
				other: $$0,
				get value() {
					return x;
				},
				set value($$value) {
					x = $$value;
					$$settled = false;
				}
			});
		});
	}
	do {
		$$settled = true;
		$$inner_renderer = $$renderer.copy();
		$$render_inner($$inner_renderer);
	} while (!$$settled);
	$$renderer.subsume($$inner_renderer);
}
