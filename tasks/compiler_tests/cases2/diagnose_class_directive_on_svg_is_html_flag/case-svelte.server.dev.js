App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { active = false } = $$props;
		$$renderer.push(`<svg${$.attr_class("icon", void 0, { "active": active })}>`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<path d="M0 0">`);
		$.push_element($$renderer, "path", 6, 4);
		$$renderer.push(`</path>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
