App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<span data-xxx="&amp;copy=value" style="&amp;copy=value">`);
		$.push_element($$renderer, "span", 1, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="©" style="©">`);
		$.push_element($$renderer, "span", 2, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="©=value" style="©=value">`);
		$.push_element($$renderer, "span", 3, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="&amp;copyotherstring=value" style="&amp;copyotherstring=value">`);
		$.push_element($$renderer, "span", 4, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="&amp;copy123=value" style="&amp;copy123=value">`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="&amp;rect=value" style="&amp;rect=value">`);
		$.push_element($$renderer, "span", 6, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <span data-xxx="▭=value" style="▭=value">`);
		$.push_element($$renderer, "span", 7, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
