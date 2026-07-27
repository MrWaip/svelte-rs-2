import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function delay(value) {
			return Promise.resolve(value);
		}
		var loaded;
		var $$promises = $$renderer.run([async () => loaded = await delay(1)]);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			Child($$renderer, { value: loaded });
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
