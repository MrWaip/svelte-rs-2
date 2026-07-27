App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let ratings = [0, 1];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(ratings);
		for (let index = 0, $$length = each_array.length; index < $$length; index++) {
			let value = each_array[index];
			Child($$renderer, { onChange: (v) => ratings[index] = v });
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
