App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		let $$derived_array = $.derived(() => $.to_array(items)), first = $.derived(() => $$derived_array()[0]), second = $.derived(() => $$derived_array()[1]), rest = $.derived(() => $$derived_array().slice(2));
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.push(`${$.escape(first())},${$.escape(second())}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
