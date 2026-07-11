App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let query = "";
		let name = "";
		function upd() {
			query = "a";
			name = "b";
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", name)}/>`);
		$.push_element($$renderer, "input", 11, 0);
		$.pop_element();
		$$renderer.push(` <textarea>`);
		$.push_element($$renderer, "textarea", 12, 0);
		const $$body = $.escape(query);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
