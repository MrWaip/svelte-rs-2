App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="a svelte-1o5tamq">`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<div class="b svelte-1o5tamq">`);
		$.push_element($$renderer, "div", 2, 1);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
