App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let counter = 0;
		let active = false;
		function getHandler() {
			return () => counter++;
		}
		$$renderer.push(`<div${$.attr_class($.clsx({ big: counter > 10 }), void 0, { "active": active })}>`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`content</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
