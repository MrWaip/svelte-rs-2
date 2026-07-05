App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="foo:bar svelte-os1qct">`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`class</div>`);
		$.pop_element();
		$$renderer.push(` <div id="hero:id" class="svelte-os1qct">`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`id</div>`);
		$.pop_element();
		$$renderer.push(` <div class="miss">`);
		$.push_element($$renderer, "div", 8, 0);
		$$renderer.push(`outside</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
