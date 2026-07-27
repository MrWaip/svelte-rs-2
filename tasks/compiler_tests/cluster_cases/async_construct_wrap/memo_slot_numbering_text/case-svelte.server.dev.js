import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function fn() {
			return 1;
		}
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 2, 0);
		$$renderer.push(async () => $.escape((await $.save("x"))()));
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(`<div${$.attr("id", fn())}>`);
		$.push_element($$renderer, "div", 2, 22);
		$$renderer.push(`y</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
