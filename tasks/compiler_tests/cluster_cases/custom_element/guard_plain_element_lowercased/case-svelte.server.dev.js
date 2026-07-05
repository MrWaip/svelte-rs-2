App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		$$renderer.push(`<div${$.attr("camelcase", obj)}>`);
		$.push_element($$renderer, "div", 4, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { obj });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
