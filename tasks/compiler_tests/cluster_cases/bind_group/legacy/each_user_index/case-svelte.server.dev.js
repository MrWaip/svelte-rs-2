App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected_array = $$props["selected_array"];
		let values = $$props["values"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(selected_array);
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let _ = each_array[$$index_1];
			let index = $$index_1;
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like(values);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let value = each_array_1[$$index];
				$$renderer.push(`<input type="checkbox"${$.attr("value", value)}${$.attr("checked", selected_array[index].includes(value), true)}/>`);
				$.push_element($$renderer, "input", 9, 2);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			selected_array,
			values
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
