App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = [{ a: 1 }, [2, 3]];
		let $$derived_array = $.derived(() => $.to_array(x, 2)), $$derived_array_1 = $.derived(() => $.to_array($$derived_array()[1], 2)), a = $.derived(() => $$derived_array()[0].a), b = $.derived(() => $$derived_array_1()[0]), c = $.derived(() => $$derived_array_1()[1]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(a())}${$.escape(b())}${$.escape(c())}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
