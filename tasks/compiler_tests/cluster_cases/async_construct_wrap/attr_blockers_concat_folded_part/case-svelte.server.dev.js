import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a, b, c;
		var $$promises = $$renderer.run([
			() => Promise.resolve(),
			() => a = "a",
			() => Promise.resolve(),
			() => b = "b",
			() => Promise.resolve(),
			() => c = "c"
		]);
		$$renderer.async([$$promises[5], $$promises[3]], ($$renderer) => {
			$$renderer.push(`<div${$.attr_style("w: a", { color: c })}${$.attr("title", b)}>`);
			$.push_element($$renderer, "div", 6, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
