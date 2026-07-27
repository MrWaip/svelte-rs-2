import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.child(async ($$renderer) => {
			const $$0 = $.clsx((await $.save("a"))());
			$$renderer.push(`<div${$.attr_class($$0, void 0, { "b": true })}>`);
			$.push_element($$renderer, "div", 1, 0);
			$$renderer.push(`y</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
