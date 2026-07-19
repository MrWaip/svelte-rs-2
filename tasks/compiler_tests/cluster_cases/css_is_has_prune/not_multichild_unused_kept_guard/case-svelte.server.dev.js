App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<a class="svelte-190fkm3">`);
		$.push_element($$renderer, "a", 1, 0);
		$$renderer.push(`<b>`);
		$.push_element($$renderer, "b", 1, 3);
		$$renderer.push(`b</b>`);
		$.pop_element();
		$$renderer.push(`</a>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
