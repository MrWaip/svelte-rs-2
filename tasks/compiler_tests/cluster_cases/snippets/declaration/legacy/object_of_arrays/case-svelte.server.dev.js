App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(s);
function s($$renderer, { p: [a, b], q: [c, d] }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<button>`);
	$.push_element($$renderer, "button", 6, 1);
	$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let v = {
			p: [1, 2],
			q: [3, 4]
		};
		s($$renderer, v);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
