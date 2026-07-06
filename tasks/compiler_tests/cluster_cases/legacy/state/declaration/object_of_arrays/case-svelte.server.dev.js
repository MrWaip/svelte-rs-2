App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			p: [1, 2],
			q: [3, 4]
		}, $$array = $.to_array(tmp.p, 2), $$array_1 = $.to_array(tmp.q, 2), a = $$array[0], b = $$array[1], c = $$array_1[0], d = $$array_1[1];
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
