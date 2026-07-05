import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(content);
function content($$renderer, value) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`${$.escape(value)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var data;
		var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			content($$renderer, data);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
