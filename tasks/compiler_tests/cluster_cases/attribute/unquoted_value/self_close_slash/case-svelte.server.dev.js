App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 1, 0);
		$$renderer.push(`<circle cx="50" cy="50" r="50">`);
		$.push_element($$renderer, "circle", 2, 1);
		$$renderer.push(`</circle>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
