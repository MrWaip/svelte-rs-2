App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(a);
function a($$renderer) {
	$.validate_snippet_args($$renderer);
	b($$renderer);
	$$renderer.push(`<!----> <div class="svelte-ile0r9">`);
	$.push_element($$renderer, "div", 3, 4);
	b($$renderer);
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
$.prevent_snippet_stringification(b);
function b($$renderer) {
	$.validate_snippet_args($$renderer);
	a($$renderer);
	$$renderer.push(`<!----> <div class="svelte-ile0r9">`);
	$.push_element($$renderer, "div", 10, 4);
	a($$renderer);
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
