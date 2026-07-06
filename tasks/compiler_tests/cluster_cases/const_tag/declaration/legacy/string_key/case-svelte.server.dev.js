App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let pairs = {
			"a-b": 1,
			"c d": 2
		};
		if (pairs) {
			$$renderer.push("<!--[0-->");
			const { "a-b": ab, "c d": cd } = pairs;
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 7, 1);
			$$renderer.push(`${$.escape(ab)}${$.escape(cd)}</button>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
