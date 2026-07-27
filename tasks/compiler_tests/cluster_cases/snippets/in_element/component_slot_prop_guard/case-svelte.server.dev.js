App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		{
			$.prevent_snippet_stringification(foo);
			function foo($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<b>`);
				$.push_element($$renderer, "b", 1, 27);
				$$renderer.push(`hi</b>`);
				$.pop_element();
			}
			Component($$renderer, {
				foo,
				$$slots: { foo: true }
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
