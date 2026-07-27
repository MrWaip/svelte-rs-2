App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = $$props["tag"];
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.push_element($$renderer, tag, 4, 0);
		$.element($$renderer, tag, void 0, () => {
			$$renderer.push(`hello`);
		});
		$.pop_element();
		$.bind_props($$props, { tag });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
