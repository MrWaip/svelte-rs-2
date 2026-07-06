App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(badge);
function badge($$renderer, text) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span class="badge">`);
	$.push_element($$renderer, "span", 10, 4);
	$$renderer.push(`${$.escape(text)}</span>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = "hello";
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<meta name="description" content="test"/>`);
			$.push_element($$renderer, "meta", 6, 4);
			$.pop_element();
		});
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 13, 0);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 14, 4);
		$$renderer.push(`hello</p>`);
		$.pop_element();
		$$renderer.push(` `);
		badge($$renderer, "new");
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
