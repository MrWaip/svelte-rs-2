App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let visible = true;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		if (visible) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 8, 8);
			$$renderer.push(`0</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 10, 8);
			$$renderer.push(`<input${$.attr("value", count)}/>`);
			$.push_element($$renderer, "input", 11, 12);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(` `);
			if (count > 10) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1>`);
				$.push_element($$renderer, "h1", 15, 12);
				$$renderer.push(`Big</h1>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 17, 12);
				$$renderer.push(`Small</h2>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 22, 0);
		if (visible) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 24, 8);
			$$renderer.push(`0</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 26, 8);
			$$renderer.push(`<input${$.attr("value", count)}/>`);
			$.push_element($$renderer, "input", 27, 12);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(` `);
			if (count > 10) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1>`);
				$.push_element($$renderer, "h1", 31, 12);
				$$renderer.push(`Big</h1>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 33, 12);
				$$renderer.push(`Small</h2>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
