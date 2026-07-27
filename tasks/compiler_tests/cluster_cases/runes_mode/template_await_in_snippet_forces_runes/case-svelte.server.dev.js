import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		async function compute(v) {
			return v * 2;
		}
		$.prevent_snippet_stringification(row);
		function row($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 10, 1);
			$$renderer.push(async () => $.escape((await $.save(compute(count)))()));
			$$renderer.push(`</p>`);
			$.pop_element();
		}
		row($$renderer);
		$$renderer.push(`<!----> <button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
