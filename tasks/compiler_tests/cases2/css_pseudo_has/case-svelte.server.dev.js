App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="card svelte-mv1sf">`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<span class="inside svelte-mv1sf">`);
		$.push_element($$renderer, "span", 6, 4);
		$$renderer.push(`inside</span>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <span class="inside">`);
		$.push_element($$renderer, "span", 9, 0);
		$$renderer.push(`outside</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
