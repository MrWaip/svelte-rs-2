App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let shown = true;
		$$renderer.push(`<annotation-xml>`);
		$.push_element($$renderer, "annotation-xml", 7, 0);
		if (shown) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 9, 2);
			$$renderer.push(`fallback html</div>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></annotation-xml>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
