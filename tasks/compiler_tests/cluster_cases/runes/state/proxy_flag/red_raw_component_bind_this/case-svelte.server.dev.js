App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let elem = void 0;
		if (Inner) {
			$$renderer.push("<!--[-->");
			Inner($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
