App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "x";
		$$renderer.push(`<h3>`);
		$.push_element($$renderer, "h3", 5, 0);
		$$renderer.push(`Hello<br/>`);
		$.push_element($$renderer, "br", 5, 9);
		$.pop_element();
		$$renderer.push(`x</h3>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
