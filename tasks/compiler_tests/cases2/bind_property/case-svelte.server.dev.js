App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let indeterminate = false;
		let open = true;
		$$renderer.push(`<input type="checkbox"/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$$renderer.push(` <details${$.attr("open", open, true)}>`);
		$.push_element($$renderer, "details", 8, 0);
		$$renderer.push(`</details>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
