App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { tracker } from "./tracker";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { track: tracker.click.upgrade() });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
