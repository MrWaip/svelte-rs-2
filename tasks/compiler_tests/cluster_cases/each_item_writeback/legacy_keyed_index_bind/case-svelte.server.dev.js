App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [0];
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(items);
			for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
				let item = each_array[idx];
				Component($$renderer, {
					get item() {
						return item;
					},
					set item($$value) {
						item = $$value;
						$$settled = false;
					}
				});
			}
			$$renderer.push(`<!--]-->`);
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
