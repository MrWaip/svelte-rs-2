App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let abc = "a";
		$.prevent_snippet_stringification(a);
		function a($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<!---->a`);
		}
		$.prevent_snippet_stringification(b);
		function b($$renderer) {
			$.validate_snippet_args($$renderer);
			a($$renderer);
		}
		b($$renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
