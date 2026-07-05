App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(defaultWrapWith);
function defaultWrapWith($$renderer, mf) {
	$.validate_snippet_args($$renderer);
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { wrapWith = defaultWrapWith } = $$props;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 11, 0);
		$$renderer.push(`x</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
