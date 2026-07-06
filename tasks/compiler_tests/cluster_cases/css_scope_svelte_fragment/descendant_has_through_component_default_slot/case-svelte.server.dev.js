App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="wrap svelte-1ktsoc1">`);
		$.push_element($$renderer, "div", 5, 0);
		Child($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<p class="svelte-1ktsoc1">`);
				$.push_element($$renderer, "p", 7, 4);
				$$renderer.push(`hi</p>`);
				$.pop_element();
			}),
			$$slots: { default: true }
		});
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
