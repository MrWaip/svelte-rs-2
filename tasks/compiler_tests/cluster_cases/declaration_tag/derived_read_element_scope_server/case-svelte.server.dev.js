App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let outer = $.derived(() => Date.now());
		{
			let inner = $.derived(() => Date.now());
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 1, 0);
			$$renderer.push(`${$.escape(inner)}</div>`);
			$.pop_element();
		}
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 3, 0);
		$$renderer.push(`${$.escape(outer())}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
