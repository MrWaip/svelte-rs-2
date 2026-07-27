App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "";
		$$renderer.push(`<set-property-before-mounted${$.attr("property", value)}>`);
		$.push_element($$renderer, "set-property-before-mounted", 5, 0);
		$$renderer.push(`</set-property-before-mounted>`);
		$.pop_element();
		$$renderer.push(` <set-property-before-mounted${$.attr("property", value)}>`);
		$.push_element($$renderer, "set-property-before-mounted", 6, 0);
		$$renderer.push(`</set-property-before-mounted>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
