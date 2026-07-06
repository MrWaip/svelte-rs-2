App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			p: { a: 1 },
			q: { b: 2 }
		}, a = $.fallback($$props["a"], () => tmp.p.a, true), b = $.fallback($$props["b"], () => tmp.q.b, true);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}</button>`);
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
