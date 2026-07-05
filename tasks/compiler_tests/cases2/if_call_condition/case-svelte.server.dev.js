App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function is_even() {
			return count % 2 === 0;
		}
		if (is_even()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 10, 1);
			$$renderer.push(`even</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 12, 1);
			$$renderer.push(`odd</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
