App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<section>`);
		$.push_element($$renderer, "section", 2, 0);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 3, 2);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 4, 4);
		$$renderer.push(`${$.escape(name)}</span>`);
		$.pop_element();
		$$renderer.push(`</span>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 7, 2);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 8, 4);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 9, 6);
		$$renderer.push(`text</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 13, 2);
		$$renderer.push(`<b>`);
		$.push_element($$renderer, "b", 14, 4);
		$$renderer.push(`<i${$.attr("name", name)}>`);
		$.push_element($$renderer, "i", 15, 6);
		$$renderer.push(`</i>`);
		$.pop_element();
		$$renderer.push(`</b>`);
		$.pop_element();
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(`</section>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
