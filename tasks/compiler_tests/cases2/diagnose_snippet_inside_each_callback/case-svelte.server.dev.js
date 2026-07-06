App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$.prevent_snippet_stringification(row);
			function row($$renderer, v) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 7, 2);
				$$renderer.push(`${$.escape(v)}</p>`);
				$.pop_element();
			}
			row($$renderer, item);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
