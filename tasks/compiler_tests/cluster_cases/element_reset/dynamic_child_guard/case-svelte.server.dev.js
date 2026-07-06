App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 5, 5);
		$$renderer.push(`${$.escape(foo)}</span>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
