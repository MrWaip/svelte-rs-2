App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		{
			$.prevent_snippet_stringification(header);
			function header($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<h1>`);
				$.push_element($$renderer, "h1", 3, 2);
				$$renderer.push(`Header</h1>`);
				$.pop_element();
			}
			$.prevent_snippet_stringification(footer);
			function footer($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 6, 2);
				$$renderer.push(`Footer</p>`);
				$.pop_element();
			}
			Card($$renderer, {
				header,
				footer,
				$$slots: {
					header: true,
					footer: true
				}
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
