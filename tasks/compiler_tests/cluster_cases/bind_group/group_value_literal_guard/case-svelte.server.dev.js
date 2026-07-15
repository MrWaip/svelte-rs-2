App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<input type="radio"${$.attr("checked", foo === false, true)}${$.attr("value", false)}/>`);
		$.push_element($$renderer, "input", 5, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", foo === true, true)}${$.attr("value", true)}/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
