App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		$.prevent_snippet_stringification(failed);
		function failed($$renderer, error) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 9, 2);
			$$renderer.push(`${$.escape(x)}: ${$.escape(error.message)}</p>`);
			$.pop_element();
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				const x = items.length;
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 7, 1);
				$$renderer.push(`${$.escape(x)}</p>`);
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
