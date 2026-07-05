App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div data-state="on" class="svelte-85638w">`);
		$.push_element($$renderer, "div", 19, 0);
		$$renderer.push(`exact</div>`);
		$.pop_element();
		$$renderer.push(` <div data-state="On" class="svelte-85638w">`);
		$.push_element($$renderer, "div", 20, 0);
		$$renderer.push(`insensitive</div>`);
		$.pop_element();
		$$renderer.push(` <div data-state="off">`);
		$.push_element($$renderer, "div", 21, 0);
		$$renderer.push(`off</div>`);
		$.pop_element();
		$$renderer.push(` <button type="button" aria-label="run" class="svelte-85638w">`);
		$.push_element($$renderer, "button", 22, 0);
		$$renderer.push(`button</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
