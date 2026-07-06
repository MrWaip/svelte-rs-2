App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let label = "hi";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.push(`<optgroup label="g">`);
		$.push_element($$renderer, "optgroup", 6, 1);
		$$renderer.push(`<em class="hdr">`);
		$.push_element($$renderer, "em", 7, 2);
		$$renderer.push(`${$.escape(label)}</em>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 8, 2);
			$$renderer.push(`A`);
			$.pop_element();
		});
		$$renderer.push(`<!></optgroup>`);
		$.pop_element();
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
