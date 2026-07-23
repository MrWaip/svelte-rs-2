App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.prevent_snippet_stringification(failed);
		function failed($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<!---->z`);
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$.prevent_snippet_stringification(a);
				function a($$renderer) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<!---->1`);
				}
				$.prevent_snippet_stringification(b);
				function b($$renderer) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<!---->2`);
				}
				$.prevent_snippet_stringification(failed);
				function failed($$renderer) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<!---->y`);
				}
				$$renderer.boundary({ failed }, ($$renderer) => {
					$$renderer.push(`<!--[-->`);
					{}
					$$renderer.push(`<!--]-->`);
				});
			}
			$$renderer.push(`<!--]-->`);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
