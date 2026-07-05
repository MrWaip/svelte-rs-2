App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let shade = "500";
		shade = "600";
		$$renderer.push(`<div${$.attr_style("", { color: `red-${$.stringify(shade)}` })}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`Concat value</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
