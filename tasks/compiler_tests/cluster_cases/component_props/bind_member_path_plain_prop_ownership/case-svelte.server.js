import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { store } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			Comp($$renderer, {
				get value() {
					return store.inner.value;
				},
				set value($$value) {
					store.inner.value = $$value;
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
	});
}
