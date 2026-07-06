App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let fruit = "apple";
		let veggie = "carrot";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 6, 0);
		$$renderer.push(`<optgroup label="Fruits">`);
		$.push_element($$renderer, "optgroup", 7, 1);
		$$renderer.push(`<span class="fh">`);
		$.push_element($$renderer, "span", 8, 2);
		$$renderer.push(`${$.escape(fruit)}</span>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 9, 2);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 9, 20);
			$$renderer.push(`${$.escape(fruit)}</span>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`<!></optgroup>`);
		$.pop_element();
		$$renderer.push(`<optgroup label="Vegs">`);
		$.push_element($$renderer, "optgroup", 11, 1);
		$$renderer.push(`<em class="vh">`);
		$.push_element($$renderer, "em", 12, 2);
		$$renderer.push(`${$.escape(veggie)}</em>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.option({ value: "c" }, ($$renderer) => {
			$.push_element($$renderer, "option", 13, 2);
			$$renderer.push(`<em>`);
			$.push_element($$renderer, "em", 13, 20);
			$$renderer.push(`${$.escape(veggie)}</em>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`<!></optgroup>`);
		$.pop_element();
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
