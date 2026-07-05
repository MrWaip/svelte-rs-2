App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let fruit = "apple";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.push(`<optgroup label="Fruits">`);
		$.push_element($$renderer, "optgroup", 6, 1);
		$$renderer.push(`<span class="hdr">`);
		$.push_element($$renderer, "span", 7, 2);
		$$renderer.push(`${$.escape(fruit)}</span>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 8, 2);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 8, 20);
			$$renderer.push(`${$.escape(fruit)}</span>`);
			$.pop_element();
			$$renderer.push(` ${$.escape(fruit)}`);
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(` `);
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$.push_element($$renderer, "option", 9, 2);
			$$renderer.push(`banana`);
			$.pop_element();
		});
		$$renderer.push(`<!></optgroup>`);
		$.pop_element();
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 13, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
