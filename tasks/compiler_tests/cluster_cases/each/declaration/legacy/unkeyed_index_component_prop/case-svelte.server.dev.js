App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let widgets = $.fallback($$props["widgets"], () => [{ name: "foo" }], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(widgets);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let widget = each_array[i];
			Widget($$renderer, {
				widget,
				index: i
			});
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { widgets });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
