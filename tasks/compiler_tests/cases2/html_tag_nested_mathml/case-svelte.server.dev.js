App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let content = "<mi>x</mi>";
		$$renderer.push(`<math>`);
		$.push_element($$renderer, "math", 5, 0);
		$$renderer.push(`<mn>`);
		$.push_element($$renderer, "mn", 6, 1);
		$$renderer.push(`1</mn>`);
		$.pop_element();
		$$renderer.push(` ${$.html(content)}</math>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
