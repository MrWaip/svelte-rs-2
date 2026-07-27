import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b } = $$props;
		$$renderer.child(async ($$renderer) => {
			const $$0 = (await $.save(a))();
			$$renderer.option({ value: `x${$.stringify($$0)}` }, ($$renderer) => {
				$.push_element($$renderer, "option", 2, 0);
				$$renderer.push(async () => $.escape((await $.save(b))()));
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
