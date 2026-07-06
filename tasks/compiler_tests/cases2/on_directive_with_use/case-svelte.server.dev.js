App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let action = $$props["action"];
		let onClick = $$props["onClick"];
		let el;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
		$.bind_props($$props, {
			action,
			onClick
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
