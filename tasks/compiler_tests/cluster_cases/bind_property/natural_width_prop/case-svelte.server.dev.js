App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let nw = $$props["nw"];
		$$renderer.push(`<img alt="x"/>`);
		$.push_element($$renderer, "img", 4, 0);
		$.pop_element();
		$.bind_props($$props, { nw });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
