App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let content = "<circle cx='5' cy='5' r='5'></circle>";
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<g>`);
		$.push_element($$renderer, "g", 6, 1);
		$$renderer.push(`</g>`);
		$.pop_element();
		$$renderer.push(`${$.html(content)}</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
