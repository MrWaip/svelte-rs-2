import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var data, handler;
		var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => handler = data.handler]);
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`hello</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
