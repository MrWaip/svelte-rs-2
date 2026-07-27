App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = "hi";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.option({}, ($$renderer) => {
			$.push_element($$renderer, "option", 5, 8);
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 5, 16);
			$$renderer.push(`${$.escape(x)}</b>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <select>`);
		$.push_element($$renderer, "select", 6, 0);
		$$renderer.option({}, ($$renderer) => {
			$.push_element($$renderer, "option", 6, 8);
			$$renderer.push(`<i>`);
			$.push_element($$renderer, "i", 6, 16);
			$$renderer.push(`${$.escape(x)}</i>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 8, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
