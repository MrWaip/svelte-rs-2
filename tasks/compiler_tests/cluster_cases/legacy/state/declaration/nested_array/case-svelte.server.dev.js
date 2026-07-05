App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = [[1, 2], [3, 4]], $$array = $.to_array(tmp, 2), $$array_1 = $.to_array($$array[0], 2), $$array_2 = $.to_array($$array[1], 2), a = $$array_1[0], b = $$array_1[1], c = $$array_2[0], d = $$array_2[1];
		function bump() {
			a = a;
			b = b;
			c = c;
			d = d;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
