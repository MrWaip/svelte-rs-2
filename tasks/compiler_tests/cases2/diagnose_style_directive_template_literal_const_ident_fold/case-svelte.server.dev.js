App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const H = 8;
		let w = 0;
		setTimeout(() => {
			w = 10;
		});
		$$renderer.push(`<div${$.attr_style("", {
			height: "8px",
			width: `${$.stringify(w)}px`
		})}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
