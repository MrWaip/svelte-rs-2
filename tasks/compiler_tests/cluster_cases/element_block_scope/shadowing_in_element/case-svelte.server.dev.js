App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 1;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 4, 0);
		$$renderer.push(`${$.escape(x)}</button>`);
		$.pop_element();
		$$renderer.push(` `);
		{
			const x = "inner";
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 5, 0);
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 7, 1);
			$$renderer.push(`inner</b>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
