App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = $$props["value"];
		const onClick = (v) => () => {
			value = v;
		};
		$$renderer.push(`<div${$.attr_class("chip", void 0, { "active": value === 1 })}>`);
		$.push_element($$renderer, "div", 11, 0);
		$$renderer.push(`hi</div>`);
		$.pop_element();
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
