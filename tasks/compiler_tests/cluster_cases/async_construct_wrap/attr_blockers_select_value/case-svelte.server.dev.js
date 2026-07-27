import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.select({ value: a }, ($$renderer) => {
				$$renderer.option({}, ($$renderer) => {
					$.push_element($$renderer, "option", 2, 18);
					$$renderer.push(`x`);
					$.pop_element();
				});
			});
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
