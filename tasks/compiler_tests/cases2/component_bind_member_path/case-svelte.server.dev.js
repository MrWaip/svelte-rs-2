App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let store = { inner: { value: 0 } };
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
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
