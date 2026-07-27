import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function getValue() {
			return loaded + value;
		}
		function setValue(v) {
			value = v;
		}
		var loaded, value;
		var $$promises = $$renderer.run([async () => loaded = await Promise.resolve(1), () => value = ""]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<input${$.attr("value", getValue())}/>`);
			$.push_element($$renderer, "input", 14, 0);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
