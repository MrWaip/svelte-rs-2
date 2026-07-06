App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "world";
		{
			$.prevent_snippet_stringification(title);
			function title($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 7, 2);
				$$renderer.push(`Hello</h2>`);
				$.pop_element();
			}
			Card($$renderer, {
				title,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 9, 1);
					$$renderer.push(`Content world</p>`);
					$.pop_element();
				}),
				$$slots: {
					title: true,
					default: true
				}
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
