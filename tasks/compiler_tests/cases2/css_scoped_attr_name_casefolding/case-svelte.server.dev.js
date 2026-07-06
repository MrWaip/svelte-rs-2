App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div data-role="banner" class="svelte-1k56wwr">`);
		$.push_element($$renderer, "div", 15, 0);
		$$renderer.push(`banner</div>`);
		$.pop_element();
		$$renderer.push(` <button type="button" aria-label="run" class="svelte-1k56wwr">`);
		$.push_element($$renderer, "button", 16, 0);
		$$renderer.push(`run</button>`);
		$.pop_element();
		$$renderer.push(` <svg viewBox="0 0 10 10" class="svelte-1k56wwr">`);
		$.push_element($$renderer, "svg", 17, 0);
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
