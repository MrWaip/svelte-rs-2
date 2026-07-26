App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rows = $$props["rows"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let [first, second, ...[third, ...{ length }]] = each_array[$$index];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 1);
			$$renderer.push(`${$.escape(first)}${$.escape(second)}</span>`);
			$.pop_element();
			$$renderer.push(` <input${$.attr("value", third)}/>`);
			$.push_element($$renderer, "input", 8, 1);
			$.pop_element();
			$$renderer.push(` <input${$.attr("value", length)}/>`);
			$.push_element($$renderer, "input", 9, 1);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { rows });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
