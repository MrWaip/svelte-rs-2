App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "";
		let name = "";
		$$renderer.push(`<input${$.attr("value", value)}/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", name)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", name)}/>`);
		$.push_element($$renderer, "input", 19, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
