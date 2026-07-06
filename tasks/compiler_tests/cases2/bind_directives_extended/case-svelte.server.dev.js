App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "";
		let checked = false;
		let group = void 0;
		$$renderer.push(`<input${$.attr("value", value)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", value)}/>`);
		$.push_element($$renderer, "input", 10, 0);
		$.pop_element();
		$$renderer.push(` <input type="checkbox"${$.attr("checked", checked, true)}/>`);
		$.push_element($$renderer, "input", 12, 0);
		$.pop_element();
		$$renderer.push(` <input type="checkbox"${$.attr("checked", checked, true)}/>`);
		$.push_element($$renderer, "input", 14, 0);
		$.pop_element();
		$$renderer.push(` <input/>`);
		$.push_element($$renderer, "input", 16, 0);
		$.pop_element();
		$$renderer.push(` <input/>`);
		$.push_element($$renderer, "input", 18, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
