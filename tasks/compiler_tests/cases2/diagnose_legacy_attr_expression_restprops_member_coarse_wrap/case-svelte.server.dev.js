App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["name"]);
	$$renderer.component(($$renderer) => {
		let name = $.fallback($$props["name"], "n");
		$$renderer.push(`<div${$.attr("id", $$restProps.id || name)}>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { name });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
