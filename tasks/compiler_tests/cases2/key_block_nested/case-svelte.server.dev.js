App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		count = 1;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`before <!---->`);
		{
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 9, 4);
			$$renderer.push(`${$.escape(count)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!----> after</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
