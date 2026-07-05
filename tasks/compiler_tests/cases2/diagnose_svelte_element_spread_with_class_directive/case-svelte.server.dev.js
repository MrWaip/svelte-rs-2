App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let props = { id: "bar" };
		let active = false;
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.push_element($$renderer, tag, 7, 0);
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attributes({ ...props }, void 0, { active })}`);
		}, () => {
			$$renderer.push(`x`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
