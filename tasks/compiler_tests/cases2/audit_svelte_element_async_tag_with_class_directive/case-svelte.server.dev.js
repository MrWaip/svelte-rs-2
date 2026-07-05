import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let active = false;
		async function getTag() {
			return "div";
		}
		const $$tag = (await $.save(getTag()))();
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$$renderer.child_block(async ($$renderer) => {
			$.push_element($$renderer, $$tag, 6, 0);
			$.element($$renderer, $$tag, () => {
				$$renderer.push(`${$.attr_class("", void 0, { "active": active })}`);
			}, () => {
				$$renderer.push(`x`);
			});
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
