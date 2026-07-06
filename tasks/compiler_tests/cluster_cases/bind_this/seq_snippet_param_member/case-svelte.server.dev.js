App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(funBind);
function funBind($$renderer, context) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<input/>`);
	$.push_element($$renderer, "input", 4, 1);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		funBind($$renderer, { set element(e) {} });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
