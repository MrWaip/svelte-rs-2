App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { noop } from "./helpers.js";
$.prevent_snippet_stringification(socket);
function socket($$renderer) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div>`);
	$.push_element($$renderer, "div", 6, 1);
	$$renderer.push(`${$.escape(noop)}</div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		socket($$renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
