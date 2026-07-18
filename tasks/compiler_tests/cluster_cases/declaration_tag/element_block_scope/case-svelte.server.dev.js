App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { cls } = $$props;
		const active = cls === "on";
		$$renderer.push(`<div${$.attr_class($.clsx(active ? "a" : "b"))}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`${$.escape(active)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
