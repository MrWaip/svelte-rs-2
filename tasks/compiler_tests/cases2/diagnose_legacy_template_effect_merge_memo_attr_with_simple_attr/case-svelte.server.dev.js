App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = $.fallback($$props["value"], 0);
		let label = $.fallback($$props["label"], "");
		function toPx(n) {
			return n + "px";
		}
		$$renderer.push(`<div${$.attr_style(`--w: ${$.stringify(toPx(value))};`)}${$.attr("data-testid", label)}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, {
			value,
			label
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
