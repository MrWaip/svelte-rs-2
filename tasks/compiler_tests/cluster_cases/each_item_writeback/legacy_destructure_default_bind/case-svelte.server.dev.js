App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let array = $.fallback($$props["array"], () => [{ value: "" }, {}], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(array);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let { value = "hello" } = each_array[$$index];
			$$renderer.push(`<input${$.attr("value", value)}/>`);
			$.push_element($$renderer, "input", 6, 1);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { array });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
