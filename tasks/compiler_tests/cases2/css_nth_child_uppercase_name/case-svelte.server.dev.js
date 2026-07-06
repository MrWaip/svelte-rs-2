App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<ul>`);
		$.push_element($$renderer, "ul", 1, 0);
		$$renderer.push(`<li class="a svelte-ym30ly">`);
		$.push_element($$renderer, "li", 1, 4);
		$$renderer.push(`x</li>`);
		$.pop_element();
		$$renderer.push(`</ul>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
