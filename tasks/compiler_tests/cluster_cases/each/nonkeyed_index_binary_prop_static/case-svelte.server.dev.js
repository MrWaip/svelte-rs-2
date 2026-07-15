App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let list = $$props["list"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(list || []);
		for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
			let item = each_array[idx];
			Child($$renderer, { label: `ID (${$.stringify(idx + 1)})` });
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { list });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
