App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
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
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
