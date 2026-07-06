App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`text only</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 3, 0);
		$$renderer.push(`${$.escape(interpolation)}</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`concatenated + ${$.escape(interpolation)} + concatenated</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 11, 0);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 12, 4);
		$$renderer.push(`more nested</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 13, 4);
		$$renderer.push(`more nested</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 14, 4);
		$$renderer.push(`more nested</div>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 17, 0);
		if (1 !== 1) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 20, 4);
			$$renderer.push(`</div>`);
			$.pop_element();
		} else if (2 === 2) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 22, 4);
			$$renderer.push(`</div>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 24, 4);
			$$renderer.push(`</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 29, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
