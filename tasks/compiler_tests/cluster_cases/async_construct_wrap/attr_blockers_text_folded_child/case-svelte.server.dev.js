import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var msg;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => msg = "hi"]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 2, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(msg)));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
