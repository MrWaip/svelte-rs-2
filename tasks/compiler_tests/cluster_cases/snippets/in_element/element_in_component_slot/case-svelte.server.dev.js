App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Component($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$.prevent_snippet_stringification(foo);
				function foo($$renderer) {
					$.validate_snippet_args($$renderer);
					$$renderer.push(`<b>`);
					$.push_element($$renderer, "b", 1, 32);
					$$renderer.push(`hi</b>`);
					$.pop_element();
				}
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 1, 11);
				$$renderer.push(`</div>`);
				$.pop_element();
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
