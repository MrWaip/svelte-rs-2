App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(s);
function s($$renderer, { p: { a }, q: { b } }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<button>`);
	$.push_element($$renderer, "button", 5, 1);
	$$renderer.push(`${$.escape(a)}${$.escape(b)}</button>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let v = {
			p: { a: 1 },
			q: { b: 2 }
		};
		s($$renderer, v);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
