import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var response;
		var $$promises = $$renderer.run([async () => response = await fetch("/api")]);
		$$renderer.async([$$promises[0]], async ($$renderer) => {
			const $$0 = (await $.save(response.text()))();
			$$renderer.push(`<div${$.attr("title", $$0)}>`);
			$.push_element($$renderer, "div", 5, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
