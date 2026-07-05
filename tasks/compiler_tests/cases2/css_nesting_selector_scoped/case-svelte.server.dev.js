App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="card svelte-1u8u4ji">`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`<h2 class="title svelte-1u8u4ji">`);
		$.push_element($$renderer, "h2", 11, 4);
		$$renderer.push(`inside</h2>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <section class="panel svelte-1u8u4ji">`);
		$.push_element($$renderer, "section", 14, 0);
		$$renderer.push(`<h3 class="label svelte-1u8u4ji">`);
		$.push_element($$renderer, "h3", 15, 4);
		$$renderer.push(`implicit</h3>`);
		$.pop_element();
		$$renderer.push(`</section>`);
		$.pop_element();
		$$renderer.push(` <h2 class="title">`);
		$.push_element($$renderer, "h2", 18, 0);
		$$renderer.push(`outside</h2>`);
		$.pop_element();
		$$renderer.push(` <h3 class="label">`);
		$.push_element($$renderer, "h3", 19, 0);
		$$renderer.push(`outside implicit</h3>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
