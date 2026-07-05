App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.prevent_snippet_stringification(failed);
		function failed($$renderer, error) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 9, 2);
			$$renderer.push(`${$.escape(error.message)}</p>`);
			$.pop_element();
			$$renderer.push(` `);
			helper($$renderer);
			$$renderer.push(`<!---->`);
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$.prevent_snippet_stringification(helper);
				function helper($$renderer) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<span>`);
					$.push_element($$renderer, "span", 5, 2);
					$$renderer.push(`helper text</span>`);
					$.pop_element();
				}
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 2, 1);
				$$renderer.push(`content</p>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
