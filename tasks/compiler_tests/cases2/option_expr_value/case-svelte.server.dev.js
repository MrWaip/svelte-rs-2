App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "a";
		function pick() {
			value = "b";
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`pick</button>`);
		$.pop_element();
		$$renderer.push(` <select>`);
		$.push_element($$renderer, "select", 7, 0);
		$$renderer.option({ value }, ($$renderer) => {
			$.push_element($$renderer, "option", 8, 1);
			$$renderer.push(`A`);
			$.pop_element();
		});
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
