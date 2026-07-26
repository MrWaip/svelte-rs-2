import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function delay(value) {
			return Promise.resolve(value);
		}
		var loaded, x;
		var $$promises = $$renderer.run([async () => loaded = await delay(1), () => x = 0]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.async_block([
			$$promises[0],
			$$promises[0],
			$$promises[1]
		], async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			Child($$renderer, {
				a: loaded,
				b: $$0
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
