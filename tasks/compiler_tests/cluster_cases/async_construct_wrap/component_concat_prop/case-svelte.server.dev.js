import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function f() {
			return 1;
		}
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(f()))();
			$.css_props($$renderer, true, { "--c": "1px" }, () => {
				Child($$renderer, { a: `y${$.stringify($$0)}` });
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
