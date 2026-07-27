import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var color, width;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => {
			color = "red";
			width = "1px";
		}]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<div${$.attr_style("", { color })}>`);
			$.push_element($$renderer, "div", 7, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<div${$.attr_style("", { width })}>`);
			$.push_element($$renderer, "div", 8, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
