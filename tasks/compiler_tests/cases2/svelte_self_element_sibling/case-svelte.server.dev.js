App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 1;
		if (count > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 6, 1);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 6);
			$$renderer.push(`ok</p>`);
			$.pop_element();
			App($$renderer, {});
			$$renderer.push(`<!----></div>`);
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
