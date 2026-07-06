App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { slide } from "svelte/transition";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let visible = true;
		function k() {}
		function c() {}
		if (visible) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 9, 1);
			$$renderer.push(`hi</div>`);
			$.pop_element();
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
