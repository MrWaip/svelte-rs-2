import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 2, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.option({ value: a }, ($$renderer) => {
				$.push_element($$renderer, "option", 2, 8);
				$$renderer.push(`x`);
				$.pop_element();
			});
		});
		$$renderer.push(`</select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
