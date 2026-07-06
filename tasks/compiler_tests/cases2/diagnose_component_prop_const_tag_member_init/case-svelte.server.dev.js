App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { p } = $$props;
		const store = { sel: { y: 1 } };
		if (true) {
			$$renderer.push("<!--[0-->");
			const a = store.sel;
			Comp($$renderer, { foo: a.y });
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
