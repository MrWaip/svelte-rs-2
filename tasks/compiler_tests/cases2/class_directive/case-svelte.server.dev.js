App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let absolute = void 0;
		let visible = void 0;
		let unchanged = void 0;
		let untouched = void 0;
		const staticClass = true;
		visible = 12;
		absolute = true;
		$$renderer.push(`<div${$.attr_class("", void 0, {
			"visible": visible,
			"absolute": absolute,
			"unchanged": unchanged,
			"untouched": untouched,
			"staticClass": staticClass,
			"static2": true
		})}>`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`Lorem</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
