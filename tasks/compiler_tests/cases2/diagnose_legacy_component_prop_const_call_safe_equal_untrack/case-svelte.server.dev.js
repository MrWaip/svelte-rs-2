App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $.fallback($$props["x"], 0);
		const tracker = { click: () => 1 };
		Child($$renderer, {
			left: tracker.click(),
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<!---->${$.escape(x)}`);
			}),
			$$slots: { default: true }
		});
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
