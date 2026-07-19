App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="a svelte-1n8hfqt">`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<span class="b svelte-1n8hfqt">`);
		$.push_element($$renderer, "span", 1, 15);
		$$renderer.push(`s</span>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
