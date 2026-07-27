import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var x;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => x = 2]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<div${$.attr("title", x)} class="a2b">`);
			$.push_element($$renderer, "div", 8, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
		$$renderer.push(` `);
		$$renderer.async_block([$$promises[1]], ($$renderer) => {
			Child($$renderer, {
				a: () => x,
				b: { k: x }
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
