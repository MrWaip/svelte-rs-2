App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<section>`);
		$.push_element($$renderer, "section", 1, 0);
		$$renderer.push(`Раздел <span>`);
		$.push_element($$renderer, "span", 1, 16);
		$$renderer.push(`Внутри <em>`);
		$.push_element($$renderer, "em", 1, 29);
		$$renderer.push(`текст</em>`);
		$.pop_element();
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(`</section>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
