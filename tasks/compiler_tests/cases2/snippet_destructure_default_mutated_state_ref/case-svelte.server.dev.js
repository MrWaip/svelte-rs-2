App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let counter = 0;
		counter = 10;
		$.prevent_snippet_stringification(row);
		function row($$renderer, { values = [counter] }) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 1);
			$$renderer.push(`${$.escape(values.length)}</span>`);
			$.pop_element();
		}
		row($$renderer, {});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
