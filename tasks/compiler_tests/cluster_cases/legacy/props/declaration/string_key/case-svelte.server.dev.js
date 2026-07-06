App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = {
			"a-b": 1,
			"c d": 2
		}, ab = $.fallback($$props["ab"], () => tmp["a-b"], true), cd = $.fallback($$props["cd"], () => tmp["c d"], true);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`${$.escape(ab)}${$.escape(cd)}</button>`);
		$.pop_element();
		$.bind_props($$props, {
			ab,
			cd
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
