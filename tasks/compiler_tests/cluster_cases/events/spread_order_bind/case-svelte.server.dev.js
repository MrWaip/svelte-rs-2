App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = $.fallback($$props["rest"], () => ({}), true);
		let value = $.fallback($$props["value"], "");
		$$renderer.push(`<input${$.attributes({
			...rest,
			value
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 5, 0);
		$.pop_element();
		$.bind_props($$props, {
			rest,
			value
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
