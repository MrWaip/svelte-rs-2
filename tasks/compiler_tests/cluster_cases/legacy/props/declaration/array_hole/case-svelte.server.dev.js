App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = [
			1,
			2,
			3
		], $$array = $.to_array(tmp, 3), a = $.fallback($$props["a"], () => $$array[0], true), c = $.fallback($$props["c"], () => $$array[2], true);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(c)}</button>`);
		$.pop_element();
		$.bind_props($$props, {
			a,
			c
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
