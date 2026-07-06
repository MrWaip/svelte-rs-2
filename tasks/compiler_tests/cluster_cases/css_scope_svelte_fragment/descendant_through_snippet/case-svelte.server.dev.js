App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(row);
function row($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p class="svelte-5iy3wu">`);
	$.push_element($$renderer, "p", 3, 16);
	$$renderer.push(`hi</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="wrap svelte-5iy3wu">`);
		$.push_element($$renderer, "div", 1, 0);
		row($$renderer);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
