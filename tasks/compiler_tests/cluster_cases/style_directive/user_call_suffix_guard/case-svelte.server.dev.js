App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function foo(n) {
			return n + 1;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 8, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		$$renderer.push(` <div${$.attr_style("", { left: `${$.stringify(foo(x))}px` })}>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
