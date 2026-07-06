App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let todos = [{ done: false }];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(todos);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let todo = each_array[$$index];
			$$renderer.push(`<input type="checkbox"${$.attr("checked", todo.done, true)}/>`);
			$.push_element($$renderer, "input", 7, 1);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
