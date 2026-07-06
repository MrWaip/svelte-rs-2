import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function makePayload() {
		return { payoffLazy: { data: null } };
	}
	const tmp = makePayload(), payoffLazy = tmp.payoffLazy;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get payoffStore() {
				return payoffLazy.data;
			},
			set payoffStore($$value) {
				payoffLazy.data = $$value;
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
