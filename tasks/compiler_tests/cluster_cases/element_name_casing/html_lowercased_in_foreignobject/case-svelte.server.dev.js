App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 1, 0);
		$$renderer.push(`<foreignObject>`);
		$.push_element($$renderer, "foreignObject", 1, 5);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 20);
		$$renderer.push(`x</div>`);
		$.pop_element();
		$$renderer.push(`</foreignObject>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
