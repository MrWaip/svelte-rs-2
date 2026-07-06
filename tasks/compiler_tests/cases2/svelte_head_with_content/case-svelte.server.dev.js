App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<meta name="description" content="A great app"/>`);
			$.push_element($$renderer, "meta", 2, 1);
			$.pop_element();
		});
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 5, 0);
		$$renderer.push(`Hello</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
