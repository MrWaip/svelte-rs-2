App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<!---->`);
		{
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 8, 4);
			$$renderer.push(`${$.escape(item.id)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!---->`);
		$.bind_props($$props, { item });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
