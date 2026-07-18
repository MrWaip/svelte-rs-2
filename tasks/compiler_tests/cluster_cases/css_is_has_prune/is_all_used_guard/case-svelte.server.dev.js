App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<x class="svelte-1fgbrwj">`);
		$.push_element($$renderer, "x", 1, 0);
		$$renderer.push(`<y class="svelte-1fgbrwj">`);
		$.push_element($$renderer, "y", 1, 3);
		$$renderer.push(`y</y>`);
		$.pop_element();
		$$renderer.push(`<z class="svelte-1fgbrwj">`);
		$.push_element($$renderer, "z", 1, 11);
		$$renderer.push(`z</z>`);
		$.pop_element();
		$$renderer.push(`</x>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
