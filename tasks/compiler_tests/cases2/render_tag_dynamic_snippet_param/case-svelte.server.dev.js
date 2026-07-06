App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(wrapper);
function wrapper($$renderer, inner) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div>`);
	$.push_element($$renderer, "div", 6, 1);
	inner($$renderer);
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
$.prevent_snippet_stringification(greeting);
function greeting($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 10, 1);
	$$renderer.push(`Hello</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let msg = "hi";
		wrapper($$renderer, greeting);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
