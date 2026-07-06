App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = 5;
		Widget($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				const doubled = value * 2;
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 1);
				$$renderer.push(`10</p>`);
				$.pop_element();
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
