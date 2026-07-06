App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		{
			$.prevent_snippet_stringification(right);
			function right($$renderer) {
				$.validate_snippet_args($$renderer);
				Btn($$renderer, {});
			}
			Header($$renderer, {
				right,
				$$slots: { right: true }
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
