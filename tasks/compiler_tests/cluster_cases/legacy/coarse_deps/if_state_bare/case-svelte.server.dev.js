App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = 1;
		function inc() {
			foo = foo + 1;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 1, 60);
		$$renderer.push(`+</button>`);
		$.pop_element();
		if (foo) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`a`);
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
