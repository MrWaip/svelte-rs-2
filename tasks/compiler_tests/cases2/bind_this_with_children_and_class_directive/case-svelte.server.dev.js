App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let dynamicEl;
		let counter = 0;
		$$renderer.push(`<div${$.attr_class("", void 0, { "state": counter > 0 })}>`);
		$.push_element($$renderer, "div", 6, 0);
		if (counter) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 11, 8);
			$$renderer.push(`x</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
