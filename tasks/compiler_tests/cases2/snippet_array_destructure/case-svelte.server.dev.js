App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(show);
function show($$renderer, [a, b]) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`${$.escape(a)} and ${$.escape(b)}</p>`);
	$.pop_element();
}
$.prevent_snippet_stringification(withRest);
function withRest($$renderer, [first, ...others]) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 10, 1);
	$$renderer.push(`${$.escape(first)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let pair = [10, 20];
		show($$renderer, pair);
		$$renderer.push(`<!----> `);
		withRest($$renderer, [
			1,
			2,
			3
		]);
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
