App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<span class="a svelte-1oo9t8i">`);
		$.push_element($$renderer, "span", 1, 0);
		$$renderer.push(`<i class="b svelte-1oo9t8i">`);
		$.push_element($$renderer, "i", 1, 16);
		$$renderer.push(`b</i>`);
		$.pop_element();
		$$renderer.push(`</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
