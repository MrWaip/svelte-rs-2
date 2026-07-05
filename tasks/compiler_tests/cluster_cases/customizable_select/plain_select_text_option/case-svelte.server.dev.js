App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let v = "a";
		$$renderer.select({ value: v }, ($$renderer) => {
			$$renderer.option({ value: "a" }, ($$renderer) => {
				$.push_element($$renderer, "option", 6, 1);
				$$renderer.push(`A`);
				$.pop_element();
			});
			$$renderer.option({ value: "b" }, ($$renderer) => {
				$.push_element($$renderer, "option", 7, 1);
				$$renderer.push(`B`);
				$.pop_element();
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
