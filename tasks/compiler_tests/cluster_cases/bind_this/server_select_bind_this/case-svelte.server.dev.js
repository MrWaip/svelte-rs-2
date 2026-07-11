App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let ref;
		let val = "a";
		$$renderer.select({
			value: val,
			this: ref
		}, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$.push_element($$renderer, "option", 7, 1);
				$$renderer.push(`a`);
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
