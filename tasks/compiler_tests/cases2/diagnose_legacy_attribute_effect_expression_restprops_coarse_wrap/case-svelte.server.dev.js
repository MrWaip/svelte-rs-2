App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["name", "checked"]);
	$$renderer.component(($$renderer) => {
		let name = $.fallback($$props["name"], "");
		let checked = $.fallback($$props["checked"], false);
		$$renderer.push(`<input${$.attributes({
			type: "checkbox",
			checked,
			id: $$restProps.id || name,
			...$$restProps
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$.bind_props($$props, {
			name,
			checked
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
