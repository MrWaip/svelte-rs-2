App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = true;
		let tag = "div";
		if (show) {
			$$renderer.push("<!--[0-->");
			$.validate_dynamic_element_tag(() => tag);
			$.validate_void_dynamic_element(() => tag);
			$.push_element($$renderer, tag, 7, 1);
			$.element($$renderer, tag, void 0, () => {
				$$renderer.push(`content`);
			});
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
