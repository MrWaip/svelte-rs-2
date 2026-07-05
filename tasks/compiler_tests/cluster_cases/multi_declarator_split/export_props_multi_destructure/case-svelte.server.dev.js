App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $.fallback($$props["a"], 1);
		let tmp = { b: 2 }, b = $.fallback($$props["b"], () => tmp.b, true);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}</p>`);
		$.pop_element();
		$.bind_props($$props, {
			a,
			b
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
