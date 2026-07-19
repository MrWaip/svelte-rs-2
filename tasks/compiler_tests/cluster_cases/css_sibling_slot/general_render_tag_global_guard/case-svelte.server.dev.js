App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<p class="before svelte-105a842">`);
		$.push_element($$renderer, "p", 2, 1);
		$$renderer.push(`before</p>`);
		$.pop_element();
		$$renderer.push(` `);
		children($$renderer);
		$$renderer.push(`<!----> <p class="foo svelte-105a842">`);
		$.push_element($$renderer, "p", 4, 1);
		$$renderer.push(`<span class="svelte-105a842">`);
		$.push_element($$renderer, "span", 5, 2);
		$$renderer.push(`foo</span>`);
		$.pop_element();
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p class="bar svelte-105a842">`);
		$.push_element($$renderer, "p", 7, 1);
		$$renderer.push(`bar</p>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
