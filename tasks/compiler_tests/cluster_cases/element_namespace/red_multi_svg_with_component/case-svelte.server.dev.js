App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 2, 0);
		$$renderer.push(`</svg>`);
		$.pop_element();
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 2, 11);
		$$renderer.push(`</svg>`);
		$.pop_element();
		Foo($$renderer, {});
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
