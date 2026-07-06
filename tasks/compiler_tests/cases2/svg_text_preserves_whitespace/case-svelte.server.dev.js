App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let label = "hello";
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<text${$.attr("x", 10)}${$.attr("y", 20)}>`);
		$.push_element($$renderer, "text", 6, 1);
		$$renderer.push(`hello</text>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
