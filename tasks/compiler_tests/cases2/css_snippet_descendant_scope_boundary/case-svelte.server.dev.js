App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(summary);
function summary($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<section class="summary svelte-ic1cb7">`);
	$.push_element($$renderer, "section", 12, 4);
	$$renderer.push(`summary</section>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let active = true;
		$$renderer.push(`<div${$.attr_class("chunk-shell svelte-ic1cb7", void 0, { "state": active })}>`);
		$.push_element($$renderer, "div", 15, 0);
		summary($$renderer);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
