App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let props = {
			id: "a",
			style: "border-color: blue;"
		};
		let color = "red";
		$$renderer.push(`<div${$.attributes({ ...props }, void 0, void 0, { color })}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
