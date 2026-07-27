App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(co);
function co($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<b>`);
	$.push_element($$renderer, "b", 5, 15);
	$$renderer.push(`C</b>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = true;
		if (show) {
			$$renderer.push("<!--[0-->");
			co($$renderer);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <select>`);
		$.push_element($$renderer, "select", 7, 0);
		$$renderer.option({}, ($$renderer) => {
			$.push_element($$renderer, "option", 7, 8);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 16);
			$$renderer.push(`M</span>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
