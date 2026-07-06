import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 1;
		$.prevent_snippet_stringification(failed);
		function failed($$renderer, error) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 10, 2);
			$$renderer.push(`${$.escape(error.message)}</p>`);
			$.pop_element();
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				const doubled = x * 2;
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 7, 1);
				$$renderer.push(`2</p>`);
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
