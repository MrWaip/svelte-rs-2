App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="svelte-11581yn">`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`content</p>`);
		$.pop_element();
		$$renderer.push(` <section class="svelte-11581yn">`);
		$.push_element($$renderer, "section", 11, 0);
		$$renderer.push(`<span class="inner">`);
		$.push_element($$renderer, "span", 12, 1);
		$$renderer.push(`inside</span>`);
		$.pop_element();
		$$renderer.push(`</section>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
