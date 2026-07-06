App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="foo svelte-mcqa8k">`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.push(`foo</p>`);
		$.pop_element();
		$$renderer.push(` <span class="baz svelte-mcqa8k">`);
		$.push_element($$renderer, "span", 12, 0);
		$$renderer.push(`baz</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
