App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = [1, 2], $$array = $.to_array(tmp, 2), x = $$array[0], y = $$array[1];
		let tmp_1 = {
			a: 1,
			b: 2
		}, a = tmp_1.a, b = tmp_1.b;
		let tmp_2 = {}, c = $.fallback(tmp_2.c, 10), d = $.fallback(tmp_2.d, 20);
		let tmp_3 = { e: { f: 1 } }, f = tmp_3.e.f;
		let tmp_4 = {
			g: 1,
			h: 2,
			i: 3
		}, g = tmp_4.g, rest = $.exclude_from_object(tmp_4, ["g"]);
		x += 1;
		a += 1;
		c += 1;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`${$.escape(x)} ${$.escape(y)} ${$.escape(a)} ${$.escape(b)} ${$.escape(c)} ${$.escape(d)} ${$.escape(f)} ${$.escape(g)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
