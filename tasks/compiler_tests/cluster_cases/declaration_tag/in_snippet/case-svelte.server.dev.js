App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(row);
function row($$renderer, item) {
	$.validate_snippet_args($$renderer);
	const label = item.name;
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 3, 1);
	$$renderer.push(`${$.escape(label)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		row($$renderer, { name: "x" });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
