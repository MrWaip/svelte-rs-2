import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a, b;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => {
			a = "a";
			b = "b";
		}]);
		$$renderer.async([$$promises[1], $$promises[1]], ($$renderer) => {
			$$renderer.push(`<div${$.attr_class("", void 0, {
				"one": a,
				"two": b
			})}>`);
			$.push_element($$renderer, "div", 7, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
