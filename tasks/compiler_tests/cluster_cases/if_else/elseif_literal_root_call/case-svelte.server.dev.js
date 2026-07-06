App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		if ("Eva".startsWith("E")) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`eee`);
		} else if (x) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`def`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`rrr`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
