App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="box-inner svelte-1ti3mv8">`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<div data-margin-right="auto">`);
		$.push_element($$renderer, "div", 2, 4);
		$$renderer.push(`a</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
