App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let id;
		let value = $$props["value"];
		let group = $$props["group"];
		$: id = `${value}-radio`;
		$$renderer.push(`<input type="radio"${$.attr("checked", group === value, true)}${$.attr("id", id)}${$.attr("value", value)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$.bind_props($$props, {
			value,
			group
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
