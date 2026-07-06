App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let color = "red";
		let bg = "blue";
		$$renderer.push(`<div${$.attr_style("", [{ color }, { "background-color": bg }])}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`Important</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
