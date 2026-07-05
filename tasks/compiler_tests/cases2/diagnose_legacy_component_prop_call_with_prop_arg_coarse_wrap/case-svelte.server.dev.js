App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Badge from "./Badge.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		function f(n) {
			return n;
		}
		Badge($$renderer, { text: `a ${$.stringify(f(x))} b` });
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
