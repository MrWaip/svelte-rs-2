App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { initial = "a" } = $$props;
		let selected = null;
		let dyn_val = initial;
		function rotate() {
			dyn_val = dyn_val + "!";
		}
		$$renderer.push(`<input type="radio"${$.attr("checked", selected === `item-${dyn_val}`, true)}${$.attr("value", `item-${dyn_val}`)}/>`);
		$.push_element($$renderer, "input", 10, 0);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`rotate</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
