App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Inner($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { item: [a, b] }) => {
				$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}`);
			} }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
