App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected = $$props["selected"];
		let values = $$props["values"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(values);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let value = each_array[$$index];
			$$renderer.push(`<input type="checkbox"${$.attr("value", value)}${$.attr("checked", selected.includes(value), true)}/>`);
			$.push_element($$renderer, "input", 8, 1);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			selected,
			values
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
