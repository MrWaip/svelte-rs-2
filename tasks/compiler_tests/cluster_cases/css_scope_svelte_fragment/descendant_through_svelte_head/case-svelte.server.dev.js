App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<div class="h svelte-16f7pvy">`);
			$.push_element($$renderer, "div", 2, 2);
			$$renderer.push(`<p class="svelte-16f7pvy">`);
			$.push_element($$renderer, "p", 2, 17);
			$$renderer.push(`x</p>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
