App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let Holder;
		let flag = $$props["flag"];
		$: Holder = { component: Inner };
		if (flag) {
			$$renderer.push("<!--[0-->");
			Holder.component($$renderer, {});
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { flag });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
