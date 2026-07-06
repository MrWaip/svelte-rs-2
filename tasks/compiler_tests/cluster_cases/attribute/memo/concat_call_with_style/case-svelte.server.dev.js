App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let status = $.fallback($$props["status"], "neutral");
		function classify(s) {
			return s + "-x";
		}
		function widthOf(s) {
			return s.length;
		}
		$$renderer.push(`<div${$.attr_class(`slider ${$.stringify(classify(status) || "")}`)}${$.attr_style(`width: ${$.stringify(widthOf(status))}px`)}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { status });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
