import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function g() {
			return 2;
		}
		{
			$.prevent_snippet_stringification(foo);
			function foo($$renderer) {
				$.validate_snippet_args($$renderer);
				const $$tag = "div";
				$.validate_dynamic_element_tag(() => $$tag);
				$.push_element($$renderer, $$tag, 7, 2);
				$$renderer.child(async ($$renderer) => {
					const $$0 = (await $.save(g()))();
					$.element($$renderer, $$tag, () => {
						$$renderer.push(`${$.attr("title", $$0)}`);
					});
				});
				$.pop_element();
			}
			Child($$renderer, {
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
