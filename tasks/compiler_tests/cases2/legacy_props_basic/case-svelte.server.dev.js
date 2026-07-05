App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let variant = $$props["variant"];
		$$renderer.push(`<button${$.attributes({
			...$$sanitized_props,
			class: `variant-${$.stringify(variant)} ${$.stringify($$sanitized_props.class ?? "")}`
		})}>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`click me</button>`);
		$.pop_element();
		$.bind_props($$props, { variant });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
