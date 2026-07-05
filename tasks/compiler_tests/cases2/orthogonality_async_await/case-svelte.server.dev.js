import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var value;
		var $$promises = $$renderer.run([async () => value = await fetch("/api")]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(async () => $.escape((await $.save(value))())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
