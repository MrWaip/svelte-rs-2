App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(row);
function row($$renderer, c = 5) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span>`);
	$.push_element($$renderer, "span", 6, 1);
	$$renderer.push(`${$.escape(c)}</span>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 4, 0);
		$$renderer.push(`${$.escape(count)}</button>`);
		$.pop_element();
		$$renderer.push(` `);
		row($$renderer, count);
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
