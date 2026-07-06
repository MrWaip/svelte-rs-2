App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let active = $$props["active"];
		Outer($$renderer, { $$slots: { activator: ($$renderer) => {
			$$renderer.push(`<div slot="activator"${$.attr_class("", void 0, { "active": active })}>`);
			$.push_element($$renderer, "div", 8, 4);
			$$renderer.push(`hi</div>`);
			$.pop_element();
		} } });
		$.bind_props($$props, { active });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
