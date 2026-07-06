App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			a: 1,
			b: 2
		}, x = $.fallback($$props["x"], () => tmp.a, true), y = $.fallback($$props["y"], () => tmp.b, true);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(x)}${$.escape(y)}</button>`);
		$.pop_element();
		$.bind_props($$props, {
			x,
			y
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
