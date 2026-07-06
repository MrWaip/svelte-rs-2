App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { thing } from "./lib";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let m;
		let p = $$props["p"];
		$: m = p;
		$.css_props($$renderer, true, { "--color": "red" }, () => {
			Child($$renderer, { config: {
				a: m,
				b: thing
			} });
		});
		$.bind_props($$props, { p });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
