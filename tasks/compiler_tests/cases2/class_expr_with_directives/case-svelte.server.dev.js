App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let active = false;
		let highlighted = true;
		const base = "btn";
		$$renderer.push(`<div${$.attr_class($.clsx([base, active && "active"]), void 0, { "highlighted": highlighted })}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`content</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
