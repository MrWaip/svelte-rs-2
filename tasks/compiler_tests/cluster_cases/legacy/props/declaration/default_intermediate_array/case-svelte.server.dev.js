App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = [[1, 2], 3], $$array = $.to_array(tmp, 2), $$array_1 = $.to_array($.fallback($$array[0], () => [8, 9], true), 2), a = $.fallback($$props["a"], () => $$array_1[0], true), b = $.fallback($$props["b"], () => $$array_1[1], true), c = $.fallback($$props["c"], () => $$array[1], true);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
		$.pop_element();
		$.bind_props($$props, {
			a,
			b,
			c
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
