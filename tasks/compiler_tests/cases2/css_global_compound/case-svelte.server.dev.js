App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="a svelte-1jl4a2j">`);
		$.push_element($$renderer, "div", 15, 0);
		$$renderer.push(`<div class="b">`);
		$.push_element($$renderer, "div", 16, 1);
		$$renderer.push(`<div class="c">`);
		$.push_element($$renderer, "div", 17, 2);
		$$renderer.push(`compound</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <section class="svelte-1jl4a2j">`);
		$.push_element($$renderer, "section", 21, 0);
		$$renderer.push(`<strong>`);
		$.push_element($$renderer, "strong", 22, 1);
		$$renderer.push(`bare global</strong>`);
		$.pop_element();
		$$renderer.push(`</section>`);
		$.pop_element();
		$$renderer.push(` <h1 class="title">`);
		$.push_element($$renderer, "h1", 25, 0);
		$$renderer.push(`title</h1>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
