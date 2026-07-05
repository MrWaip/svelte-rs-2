App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		$$renderer.push(`<div${$.attr_class("", void 0, { "before-content": $$slots.beforeContent })}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "beforeContent", {}, null);
		$$renderer.push(`<!--]--> ${$.escape(x)}</div>`);
		$.pop_element();
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
