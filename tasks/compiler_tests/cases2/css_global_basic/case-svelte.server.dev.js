App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="svelte-r1lfc1">`);
		$.push_element($$renderer, "p", 24, 0);
		$$renderer.push(`content</p>`);
		$.pop_element();
		$$renderer.push(` <p class="active svelte-r1lfc1">`);
		$.push_element($$renderer, "p", 25, 0);
		$$renderer.push(`active</p>`);
		$.pop_element();
		$$renderer.push(` <h2>`);
		$.push_element($$renderer, "h2", 26, 0);
		$$renderer.push(`heading</h2>`);
		$.pop_element();
		$$renderer.push(` <h2 class="featured">`);
		$.push_element($$renderer, "h2", 27, 0);
		$$renderer.push(`featured</h2>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
