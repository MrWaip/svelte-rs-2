App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = $.fallback($$props["n"], 0);
		n;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 8, 0);
		$$renderer.push(`hi</div>`);
		$.pop_element();
		$.bind_props($$props, { n });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
