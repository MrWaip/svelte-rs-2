App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.component(($$renderer) => {
		const has_description = !!$$slots.description;
		if (has_description) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`has description</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
