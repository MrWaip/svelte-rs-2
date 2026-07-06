App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="host svelte-1v67kh2">`);
		$.push_element($$renderer, "div", 9, 0);
		{
			$.prevent_snippet_stringification(children);
			function children($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<span class="inside svelte-1v67kh2">`);
				$.push_element($$renderer, "span", 12, 12);
				$$renderer.push(`inside</span>`);
				$.pop_element();
			}
			Widget($$renderer, {
				children,
				$$slots: { default: true }
			});
		}
		$$renderer.push(`<!----></div>`);
		$.pop_element();
		$$renderer.push(` <span class="inside">`);
		$.push_element($$renderer, "span", 17, 0);
		$$renderer.push(`outside</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
