App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { $$slots, $$events, ...props } = $$props;
		$$renderer.push(`<button${$.attributes({
			...props,
			is: "x-button"
		}, void 0, void 0, void 0, 2)}>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`x</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
