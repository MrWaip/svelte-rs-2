App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items } = $$props;
		$.prevent_snippet_stringification(foo);
		function foo($$renderer, a) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 6, 17);
			$$renderer.push(`${$.escape(items)} ${$.escape(a)}</span>`);
			$.pop_element();
		}
		Child($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				foo($$renderer, 1);
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
