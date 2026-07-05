App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "";
		let checked = false;
		let selected = true;
		let disabled = false;
		let readonly = false;
		$$renderer.push(`<input${$.attr("value", value)}${$.attr("disabled", disabled, true)}/>`);
		$.push_element($$renderer, "input", 9, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", value)}${$.attr("readonly", readonly, true)}/>`);
		$.push_element($$renderer, "input", 10, 0);
		$.pop_element();
		$$renderer.push(` <input type="checkbox"${$.attr("checked", checked, true)}/>`);
		$.push_element($$renderer, "input", 11, 0);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.option({ selected }, ($$renderer) => {
			$.push_element($$renderer, "option", 12, 0);
			$$renderer.push(`picked`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
