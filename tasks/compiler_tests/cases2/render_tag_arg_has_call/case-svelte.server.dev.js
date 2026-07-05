App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(show);
function show($$renderer, data) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`${$.escape(data)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function getData() {
			return [
				1,
				2,
				3
			];
		}
		show($$renderer, getData());
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
