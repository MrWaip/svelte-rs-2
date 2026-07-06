App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { isExpanded } = $$props;
		$$renderer.push(`<div${$.attr_class("", void 0, { "is_expanded": isExpanded })}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`hi</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
