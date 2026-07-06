App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "x";
		$.prevent_snippet_stringification(foo);
		function foo($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<!---->x`);
		}
		foo($$renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
