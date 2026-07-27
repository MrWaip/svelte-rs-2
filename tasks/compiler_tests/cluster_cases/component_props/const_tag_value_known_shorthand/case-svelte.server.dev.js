App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let plain = 7;
		$.prevent_snippet_stringification(row);
		function row($$renderer) {
			$.validate_snippet_args($$renderer);
			const kLit = "x";
			const kArith = plain + 1;
			Child($$renderer, {
				kLit,
				kArith
			});
		}
		row($$renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
