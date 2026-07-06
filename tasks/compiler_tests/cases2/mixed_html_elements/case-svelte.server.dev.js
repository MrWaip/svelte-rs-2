App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<input/>`);
		$.push_element($$renderer, "input", 1, 0);
		$.pop_element();
		$$renderer.push(` <textarea>`);
		$.push_element($$renderer, "textarea", 2, 0);
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$$renderer.push(` <area/>`);
		$.push_element($$renderer, "area", 3, 0);
		$.pop_element();
		$$renderer.push(` <br/>`);
		$.push_element($$renderer, "br", 4, 0);
		$.pop_element();
		$$renderer.push(` <a>`);
		$.push_element($$renderer, "a", 5, 0);
		$$renderer.push(`</a>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
