App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$.prevent_snippet_stringification(failed);
		function failed($$renderer, error) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 11, 2);
			$$renderer.push(`outer: ${$.escape(error.message)}</p>`);
			$.pop_element();
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$.prevent_snippet_stringification(failed);
				function failed($$renderer, error) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 6, 3);
					$$renderer.push(`${$.escape(error.message)}</p>`);
					$.pop_element();
				}
				$$renderer.boundary({ failed }, ($$renderer) => {
					$$renderer.push(`<!--[-->`);
					{
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 3, 2);
						$$renderer.push(`inner</p>`);
						$.pop_element();
					}
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
