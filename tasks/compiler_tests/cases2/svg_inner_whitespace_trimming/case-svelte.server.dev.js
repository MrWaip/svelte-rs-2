App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let w = 100;
		let h = 100;
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 6, 0);
		$$renderer.push(`<line${$.attr("x1", 0)}${$.attr("y1", 0)}${$.attr("x2", w)}${$.attr("y2", h)}>`);
		$.push_element($$renderer, "line", 7, 1);
		$$renderer.push(`</line>`);
		$.pop_element();
		$$renderer.push(`<rect${$.attr("width", w)}${$.attr("height", h)}>`);
		$.push_element($$renderer, "rect", 8, 1);
		$$renderer.push(`</rect>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
