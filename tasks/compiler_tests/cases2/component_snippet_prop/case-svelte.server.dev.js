App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		{
			$.prevent_snippet_stringification(header);
			function header($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 7, 2);
				$$renderer.push(`Title 0</h2>`);
				$.pop_element();
			}
			Dialog($$renderer, {
				header,
				$$slots: { header: true }
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
