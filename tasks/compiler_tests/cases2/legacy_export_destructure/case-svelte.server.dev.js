App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			x: "a",
			z: ["b"]
		}, $$array = $.to_array(tmp.z, 1), foo = $.fallback($$props["foo"], () => $.fallback(tmp.x, "default-x"), true), bar = $.fallback($$props["bar"], () => $$array[0], true);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.push(`${$.escape(foo)}${$.escape(bar)}</p>`);
		$.pop_element();
		$.bind_props($$props, {
			foo,
			bar
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
