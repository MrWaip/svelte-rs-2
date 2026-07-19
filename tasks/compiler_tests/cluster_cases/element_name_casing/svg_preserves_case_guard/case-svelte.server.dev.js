App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 1, 0);
		$$renderer.push(`<clipPath id="c">`);
		$.push_element($$renderer, "clipPath", 1, 5);
		$$renderer.push(`</clipPath>`);
		$.pop_element();
		$$renderer.push(`<linearGradient id="g">`);
		$.push_element($$renderer, "linearGradient", 1, 33);
		$$renderer.push(`</linearGradient>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
