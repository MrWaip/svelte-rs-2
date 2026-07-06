App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 1, 0);
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 2, 1);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 2, 19);
			$$renderer.push(`A</span>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$.push_element($$renderer, "option", 3, 1);
			$$renderer.push(`B`);
			$.pop_element();
		});
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
