App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let model = $.fallback($$props["model"], "a");
		let value = $.fallback($$props["value"], "a");
		function action() {}
		$$renderer.push(`<input type="radio"${$.attr("value", value)}${$.attr("checked", model === value, true)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$.bind_props($$props, {
			model,
			value
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
