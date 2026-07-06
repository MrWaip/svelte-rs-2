App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $.fallback($$props["items"], () => [], true);
		Outer($$renderer, { $$slots: { cell: ($$renderer, { index, style }) => {
			const item = items[index];
			$$renderer.push(`<div slot="cell"${$.attr_style(style)}>`);
			$.push_element($$renderer, "div", 6, 2);
			$$renderer.push(`${$.escape(item)}</div>`);
			$.pop_element();
		} } });
		$.bind_props($$props, { items });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
