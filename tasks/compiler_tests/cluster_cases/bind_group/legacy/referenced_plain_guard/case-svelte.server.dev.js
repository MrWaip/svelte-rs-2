App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let test = $.fallback($$props["test"], () => [], true);
		$$renderer.push(`<label>`);
		$.push_element($$renderer, "label", 5, 0);
		$$renderer.push(`a <input type="checkbox"${$.attr("checked", test.includes("a"), true)} value="a"/>`);
		$.push_element($$renderer, "input", 5, 9);
		$.pop_element();
		$$renderer.push(`</label>`);
		$.pop_element();
		$.bind_props($$props, { test });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
