App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Button($$renderer, { label: "value: 1231" });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
