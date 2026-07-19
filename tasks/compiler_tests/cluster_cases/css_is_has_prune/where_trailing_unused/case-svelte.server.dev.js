App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<x class="svelte-zyrlym">`);
		$.push_element($$renderer, "x", 1, 0);
		$$renderer.push(`<y class="svelte-zyrlym">`);
		$.push_element($$renderer, "y", 1, 3);
		$$renderer.push(`y</y>`);
		$.pop_element();
		$$renderer.push(`</x>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
