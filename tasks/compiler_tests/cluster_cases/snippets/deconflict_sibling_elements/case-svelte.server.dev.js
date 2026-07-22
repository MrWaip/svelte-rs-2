App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.prevent_snippet_stringification(row);
		function row($$renderer, n) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 2, 18);
			$$renderer.push(`${$.escape(n)}</span>`);
			$.pop_element();
		}
		$.prevent_snippet_stringification(row);
		function row($$renderer, n) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 6, 18);
			$$renderer.push(`${$.escape(n)}</b>`);
			$.pop_element();
		}
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 0);
		row($$renderer, 1);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 5, 0);
		row($$renderer, 2);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
