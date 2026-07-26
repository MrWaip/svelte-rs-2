import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 11, 0);
			$$renderer.push(`inc</button>`);
			$.pop_element();
			$$renderer.push(` `);
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
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
