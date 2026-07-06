App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { slide } from "svelte/transition";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let visible = true;
		function k() {}
		if (visible) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 8, 1);
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 9, 2);
			$$renderer.push(`hi</button>`);
			$.pop_element();
			$$renderer.push(`</div>`);
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
