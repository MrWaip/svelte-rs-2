App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(greeting);
function greeting($$renderer, name) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`Hello ${$.escape(name)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = null;
		show($$renderer);
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
