App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let label = "hi";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.push(`<span class="hdr">`);
		$.push_element($$renderer, "span", 6, 1);
		$$renderer.push(`${$.escape(label)}</span>`);
		$.pop_element();
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$.push_element($$renderer, "option", 7, 1);
			$$renderer.push(`A`);
			$.pop_element();
		});
		$$renderer.push(`<!></select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
