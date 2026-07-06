App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = true;
		if (show) {
			$$renderer.push("<!--[0-->");
			$.prevent_snippet_stringification(failed);
			function failed($$renderer, error) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 10, 3);
				$$renderer.push(`${$.escape(error.message)}</p>`);
				$.pop_element();
			}
			$$renderer.boundary({ failed }, ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				{
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 7, 2);
					$$renderer.push(`guarded</p>`);
					$.pop_element();
				}
				$$renderer.push(`<!--]-->`);
			});
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
