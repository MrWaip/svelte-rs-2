App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let b = $$props["b"];
		let attributes = $.fallback($$props["attributes"], () => ({}), true);
		$$renderer.push(`<div${$.attributes({
			class: a + b,
			...attributes
		})}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, {
			a,
			b,
			attributes
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
