App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cond = false;
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>t</title>`);
			});
		});
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 10, 4);
			$$renderer.push(`a</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 12, 4);
			$$renderer.push(`b</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
