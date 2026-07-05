App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = {
			a: 1,
			b: []
		};
		let items = ["x", "y"];
		$$renderer.push(`<input type="radio"${$.attr("checked", data.a === 1, true)}${$.attr("value", 1)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", data.a === 2, true)}${$.attr("value", 2)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<input type="checkbox"${$.attr("checked", data.b.includes(item), true)}${$.attr("value", item)}/>`);
			$.push_element($$renderer, "input", 10, 1);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
