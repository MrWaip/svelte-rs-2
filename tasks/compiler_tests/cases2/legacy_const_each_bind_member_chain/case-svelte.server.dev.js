App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const obj = {
			keys: ["a"],
			fields: { a: { value: 0 } }
		};
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(obj.keys);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let key = each_array[$$index];
			const field = obj.fields[key];
			$$renderer.push(`<input${$.attr("value", field.value)}/>`);
			$.push_element($$renderer, "input", 7, 4);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
