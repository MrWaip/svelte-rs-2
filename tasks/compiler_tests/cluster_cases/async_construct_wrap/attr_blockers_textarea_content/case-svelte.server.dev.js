import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var value;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => value = "value"]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<textarea>`);
			$.push_element($$renderer, "textarea", 6, 0);
			const $$body = $.escape(value);
			if ($$body) {
				$$renderer.push(`${$$body}`);
			} else {}
			$$renderer.push(`</textarea>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
