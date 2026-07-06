App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<input type="text"/>`);
		$.push_element($$renderer, "input", 1, 0);
		$.pop_element();
		$$renderer.push(` <br/>`);
		$.push_element($$renderer, "br", 2, 0);
		$.pop_element();
		$$renderer.push(` <img src="test.png"/>`);
		$.push_element($$renderer, "img", 3, 0);
		$.pop_element();
		$$renderer.push(` <hr/>`);
		$.push_element($$renderer, "hr", 4, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
