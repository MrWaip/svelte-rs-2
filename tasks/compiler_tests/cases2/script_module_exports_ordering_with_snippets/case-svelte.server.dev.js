App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(row);
function row($$renderer, text) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span>`);
	$.push_element($$renderer, "span", 13, 1);
	$$renderer.push(`${$.escape(text)}</span>`);
	$.pop_element();
}
export const KIND = "v1";
export function label(name) {
	return `${KIND}:${name}`;
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { title } = $$props;
		row($$renderer, label(title));
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
