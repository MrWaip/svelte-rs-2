App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.css_props($$renderer, true, { "--color": "red" }, () => {
			Component($$renderer, {});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
