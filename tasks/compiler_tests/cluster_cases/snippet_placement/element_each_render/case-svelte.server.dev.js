App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let numbers = [
			1,
			2,
			3
		];
		$.prevent_snippet_stringification(x);
		function x($$renderer, n) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 7, 2);
			$$renderer.push(`${$.escape(n)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(numbers);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let n = each_array[$$index];
			x($$renderer, n);
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
