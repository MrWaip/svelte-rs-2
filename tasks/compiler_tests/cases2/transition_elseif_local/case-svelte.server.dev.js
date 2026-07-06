App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = false;
		let y = true;
		if (x) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 9, 1);
			$$renderer.push(`first</p>`);
			$.pop_element();
		} else if (y) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 11, 1);
			$$renderer.push(`second</div>`);
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
