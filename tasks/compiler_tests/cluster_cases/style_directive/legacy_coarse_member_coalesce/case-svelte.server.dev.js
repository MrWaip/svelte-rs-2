App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj;
		function upd(e) {
			obj = e;
		}
		$$renderer.push(`<div${$.attr_style("", { width: `${$.stringify((obj?.w || 0) + 40)}px` })}>`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
