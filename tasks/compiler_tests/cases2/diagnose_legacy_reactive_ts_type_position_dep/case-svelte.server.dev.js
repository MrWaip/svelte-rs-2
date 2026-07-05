App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import data from "./dep.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let doubled;
		let count = 0;
		$: doubled = { value: count };
		$$renderer.push(`<!---->${$.escape(doubled.value)}`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
