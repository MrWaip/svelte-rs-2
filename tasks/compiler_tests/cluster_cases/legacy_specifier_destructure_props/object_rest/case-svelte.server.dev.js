App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			a: 1,
			b: 2,
			c: 3
		}, a = $.fallback($$props["a"], () => tmp.a, true), rest = $.fallback($$props["rest"], () => $.exclude_from_object(tmp, ["a"]), true);
		function inc() {
			a++;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(JSON.stringify(rest))}</button>`);
		$.pop_element();
		$.bind_props($$props, { a });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
