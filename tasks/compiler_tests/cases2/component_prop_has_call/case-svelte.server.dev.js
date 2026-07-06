App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function getData() {
			return [
				1,
				2,
				3
			];
		}
		Comp($$renderer, { data: getData() });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
