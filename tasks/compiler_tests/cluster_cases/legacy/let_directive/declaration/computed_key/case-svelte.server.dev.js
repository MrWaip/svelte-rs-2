App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const k = "z";
		Inner($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { item: { [k]: v } }) => {
				$$renderer.push(`<!---->${$.escape(v)}`);
			} }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
