App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 1, 0);
		$$renderer.push(`<path d="M1">`);
		$.push_element($$renderer, "path", 2, 1);
		$$renderer.push(`</path>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
		$$renderer.push(`<g>`);
		$.push_element($$renderer, "g", 5, 0);
		$$renderer.push(`<path d="M2">`);
		$.push_element($$renderer, "path", 6, 1);
		$$renderer.push(`</path>`);
		$.pop_element();
		$$renderer.push(`</g>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
