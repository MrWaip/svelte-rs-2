App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let size = 1;
		let arr = [];
		function joinClasses(a) {
			return a.join(" ");
		}
		$$renderer.push(`<div${$.attributes({
			...{ id: "x" },
			class: `size_1 ${$.stringify(joinClasses(arr))}`
		})}>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
