App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let shown = true;
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 5, 0);
		if (shown) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<title>`);
			$.push_element($$renderer, "title", 7, 2);
			$$renderer.push(`Chart</title>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
