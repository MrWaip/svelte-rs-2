App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.select({ value: true }, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$.push_element($$renderer, "option", 2, 1);
				$$renderer.push(`a`);
				$.pop_element();
			});
			$$renderer.option({}, ($$renderer) => {
				$.push_element($$renderer, "option", 3, 1);
				$$renderer.push(`b`);
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
