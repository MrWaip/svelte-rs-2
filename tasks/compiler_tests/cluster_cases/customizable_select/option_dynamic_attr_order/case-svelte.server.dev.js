App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let v = "a";
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 5, 0);
		$$renderer.option({ value: v }, ($$renderer) => {
			$.push_element($$renderer, "option", 6, 1);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 6, 19);
			$$renderer.push(`${$.escape(v)}</span>`);
			$.pop_element();
			$.pop_element();
		}, void 0, void 0, void 0, void 0, true);
		$$renderer.push(`</select>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
