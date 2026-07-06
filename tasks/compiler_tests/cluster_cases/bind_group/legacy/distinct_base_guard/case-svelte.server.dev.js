App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let one = 1;
		let two = 2;
		$$renderer.push(`<input type="radio"${$.attr("checked", one === 1, true)}${$.attr("value", 1)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", two === 2, true)}${$.attr("value", 2)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
