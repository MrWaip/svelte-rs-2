App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<a href="/x">`);
		$.push_element($$renderer, "a", 2, 4);
		$$renderer.push(`link</a>`);
		$.pop_element();
		$$renderer.push(` text more `);
		if (true) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`x`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
