App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Parent($$renderer, { $$slots: { item: ($$renderer, { item }) => {
			$$renderer.push(`<div slot="item">`);
			$.push_element($$renderer, "div", 6, 1);
			$$renderer.push(`${$.escape(item)}</div>`);
			$.pop_element();
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
