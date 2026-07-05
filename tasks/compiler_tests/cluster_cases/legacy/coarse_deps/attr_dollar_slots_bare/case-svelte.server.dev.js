App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		$$renderer.push(`<p${$.attr("hidden", $$slots)}>`);
		$.push_element($$renderer, "p", 1, 30);
		$$renderer.push(`a</p>`);
		$.pop_element();
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
