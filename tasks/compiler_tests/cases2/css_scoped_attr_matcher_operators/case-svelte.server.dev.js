App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div data-tags="card active" class="svelte-v4glr4">`);
		$.push_element($$renderer, "div", 23, 0);
		$$renderer.push(`class</div>`);
		$.pop_element();
		$$renderer.push(` <div data-lang="en-US" class="svelte-v4glr4">`);
		$.push_element($$renderer, "div", 24, 0);
		$$renderer.push(`lang</div>`);
		$.pop_element();
		$$renderer.push(` <div data-url="https://example.com" class="svelte-v4glr4">`);
		$.push_element($$renderer, "div", 25, 0);
		$$renderer.push(`href</div>`);
		$.pop_element();
		$$renderer.push(` <span data-tags="inactive">`);
		$.push_element($$renderer, "span", 26, 0);
		$$renderer.push(`no class</span>`);
		$.pop_element();
		$$renderer.push(` <div data-lang="bengali">`);
		$.push_element($$renderer, "div", 27, 0);
		$$renderer.push(`no lang</div>`);
		$.pop_element();
		$$renderer.push(` <div data-url="http://sample.org">`);
		$.push_element($$renderer, "div", 28, 0);
		$$renderer.push(`no href</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
