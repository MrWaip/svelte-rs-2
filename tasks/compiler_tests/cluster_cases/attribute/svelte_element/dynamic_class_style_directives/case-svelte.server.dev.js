App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let value = "red";
		const getClass = () => value === "blue";
		const getValue = () => value;
		$.validate_dynamic_element_tag(() => tag);
		$.push_element($$renderer, tag, 8, 0);
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr_class("", void 0, { "blue": getClass() })}${$.attr_style("", { color: getValue() })}`);
		});
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`toggle</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
