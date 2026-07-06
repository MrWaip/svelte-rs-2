App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["value", "group"]);
	$$renderer.component(($$renderer) => {
		let value = $$props["value"];
		let group = $$props["group"];
		$$renderer.push(`<input${$.attributes({
			type: "radio",
			checked: group === value,
			value,
			...$$restProps
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
		$.bind_props($$props, {
			value,
			group
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
