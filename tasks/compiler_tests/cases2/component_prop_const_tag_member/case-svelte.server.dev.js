App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = { name: "a" };
		if (true) {
			$$renderer.push("<!--[0-->");
			const item = value;
			Comp($$renderer, { name: item.name });
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
