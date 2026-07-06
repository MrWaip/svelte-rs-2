App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 1, 0);
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$.push_element($$renderer, "option", 2, 1);
			$$renderer.push(`Two`);
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
