App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { header, headerTag = "div" } = $$props;
		let cond = true;
		$.validate_dynamic_element_tag(() => headerTag);
		$.validate_void_dynamic_element(() => headerTag);
		$.push_element($$renderer, headerTag, 6, 0);
		$.element($$renderer, headerTag, void 0, () => {
			header($$renderer);
			$$renderer.push(`<!----> `);
			if (cond) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 9, 2);
				$$renderer.push(`x</span>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
