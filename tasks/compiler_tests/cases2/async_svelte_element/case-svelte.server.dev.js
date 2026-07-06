import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function getTag() {
			return "div";
		}
		const $$tag = (await $.save(getTag()))();
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$$renderer.child_block(async ($$renderer) => {
			$.push_element($$renderer, $$tag, 7, 0);
			$.element($$renderer, $$tag, void 0, () => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 1);
				$$renderer.push(`content</p>`);
				$.pop_element();
			});
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
