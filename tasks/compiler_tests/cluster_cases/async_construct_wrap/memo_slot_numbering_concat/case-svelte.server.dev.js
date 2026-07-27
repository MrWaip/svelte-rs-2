import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function fn() {
			return 1;
		}
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save("x"))();
			$$renderer.push(`<div${$.attributes({
				...{},
				title: `a${$.stringify($$0)}b`,
				id: fn()
			})}>`);
			$.push_element($$renderer, "div", 2, 0);
			$$renderer.push(`y</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
