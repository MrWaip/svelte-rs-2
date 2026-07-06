App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<math>`);
		$.push_element($$renderer, "math", 3, 0);
		$$renderer.push(`<mi>`);
		$.push_element($$renderer, "mi", 4, 1);
		$$renderer.push(`x</mi>`);
		$.pop_element();
		$$renderer.push(`</math>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
