App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let model = $.fallback($$props["model"], "a");
		let value = $.fallback($$props["value"], "a");
		let name = $.fallback($$props["name"], "radio");
		let hasError = $.fallback($$props["hasError"], false);
		$$renderer.push(`<input type="radio"${$.attr("value", value)}${$.attr("name", name)}${$.attr("checked", model === value, true)}${$.attr_class("", void 0, { "error": hasError })}/>`);
		$.push_element($$renderer, "input", 9, 0);
		$.pop_element();
		$.bind_props($$props, {
			model,
			value,
			name,
			hasError
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
