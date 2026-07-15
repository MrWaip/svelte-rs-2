App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let spread = {};
		let v = void 0;
		$$renderer.push(`<input${$.attributes({
			value: v,
			...spread
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
