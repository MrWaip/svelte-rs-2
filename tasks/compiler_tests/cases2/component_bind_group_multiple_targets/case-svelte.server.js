import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let a = "x";
	let b = "y";
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get group() {
				return a;
			},
			set group($$value) {
				a = $$value;
				$$settled = false;
			}
		});
		$$renderer.push(`<!----> `);
		Child($$renderer, {
			get group() {
				return b;
			},
			set group($$value) {
				b = $$value;
				$$settled = false;
			}
		});
		$$renderer.push(`<!---->`);
	}
	do {
		$$settled = true;
		$$inner_renderer = $$renderer.copy();
		$$render_inner($$inner_renderer);
	} while (!$$settled);
	$$renderer.subsume($$inner_renderer);
}
