App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected_array = $$props["selected_array"];
		let selected_index = $$props["selected_index"];
		let options = $$props["options"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(selected_array);
		for (let $$index_2 = 0, $$length = each_array.length; $$index_2 < $$length; $$index_2++) {
			let selected = each_array[$$index_2];
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like(selected_index);
			for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
				let index = each_array_1[$$index_1];
				$$renderer.push(`<!--[-->`);
				const each_array_2 = $.ensure_array_like(options);
				for (let $$index = 0, $$length = each_array_2.length; $$index < $$length; $$index++) {
					let value = each_array_2[$$index];
					$$renderer.push(`<input type="checkbox"${$.attr("checked", selected[index].includes(value), true)}${$.attr("value", value)}/>`);
					$.push_element($$renderer, "input", 11, 3);
					$.pop_element();
				}
				$$renderer.push(`<!--]-->`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			selected_array,
			selected_index,
			options
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
