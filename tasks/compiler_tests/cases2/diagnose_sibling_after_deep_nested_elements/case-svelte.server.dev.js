App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		$.prevent_snippet_stringification(inner);
		function inner($$renderer, mf) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 6, 4);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 7, 8);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 8, 12);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 9, 16);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 9, 21);
			$$renderer.push(`a</span>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(` <div>`);
			$.push_element($$renderer, "div", 12, 8);
			$$renderer.push(`<!---->`);
			{
				mf($$renderer);
				$$renderer.push(`<!---->`);
			}
			$$renderer.push(`<!----></div>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
