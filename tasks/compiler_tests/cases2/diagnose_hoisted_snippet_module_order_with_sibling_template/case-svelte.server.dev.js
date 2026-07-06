App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(defaultWrapWith);
function defaultWrapWith($$renderer, mf) {
	$.validate_snippet_args($$renderer);
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { wrapWith = defaultWrapWith, label = "" } = $$props;
		let count = 0;
		$.prevent_snippet_stringification(inner);
		function inner($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 11, 4);
			$$renderer.push(`${$.escape(label)}0</span>`);
			$.pop_element();
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<style>
        :root { --x: red; }
    </style>`);
		});
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 14, 0);
		wrapWith($$renderer, inner);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
