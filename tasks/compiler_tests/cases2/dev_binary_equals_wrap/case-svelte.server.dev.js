App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 1;
		let b = 2;
		if (a === b) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`equal`);
		} else if (a == 1) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`one`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> true
true
true`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
