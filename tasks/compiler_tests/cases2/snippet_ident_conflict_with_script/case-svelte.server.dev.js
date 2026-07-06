App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(card);
function card($$renderer, heading) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div>`);
	$.push_element($$renderer, "div", 8, 4);
	$$renderer.push(`<h3>`);
	$.push_element($$renderer, "h3", 9, 8);
	$$renderer.push(`${$.escape(heading)}</h3>`);
	$.pop_element();
	$$renderer.push(` `);
	badge($$renderer, "new");
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function action(node, arg) {
			return { destroy() {} };
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
