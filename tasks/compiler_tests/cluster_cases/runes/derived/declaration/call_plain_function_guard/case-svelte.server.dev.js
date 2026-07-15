App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function makeValue() {
			return 42;
		}
		const value = $.derived(makeValue);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 7, 0);
		$$renderer.push(`${$.escape(value())}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
