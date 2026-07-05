App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $.fallback($$props["a"], 1);
		let b = $.fallback($$props["b"], 2);
		let c = $$props["c"];
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}</p>`);
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
