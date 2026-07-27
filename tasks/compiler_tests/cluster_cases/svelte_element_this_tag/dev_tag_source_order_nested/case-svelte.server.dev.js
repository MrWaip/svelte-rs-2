App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = false;
		const $$tag = "p";
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		const $$tag_2 = "p";
		$.validate_dynamic_element_tag(() => $$tag_2);
		$.validate_void_dynamic_element(() => $$tag_2);
		$.push_element($$renderer, $$tag, 5, 0);
		$.element($$renderer, $$tag, void 0, () => {
			$$renderer.push(`before`);
		});
		$.pop_element();
		$$renderer.push(` `);
		if (show) {
			$$renderer.push("<!--[0-->");
			const $$tag_1 = "strong";
			$.validate_dynamic_element_tag(() => $$tag_1);
			$.validate_void_dynamic_element(() => $$tag_1);
			$.push_element($$renderer, $$tag_1, 7, 1);
			$.element($$renderer, $$tag_1, void 0, () => {
				$$renderer.push(`during`);
			});
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> `);
		$.push_element($$renderer, $$tag_2, 9, 0);
		$.element($$renderer, $$tag_2, void 0, () => {
			$$renderer.push(`after`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
