App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["variant", "size"]);
	$$renderer.component(($$renderer) => {
		let variant = $.fallback($$props["variant"], "filled");
		let size = $.fallback($$props["size"], "md");
		$$renderer.push(`<button${$.attributes({
			...$$restProps,
			class: `variant-${$.stringify(variant)} size-${$.stringify(size)}`
		})}>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`click me</button>`);
		$.pop_element();
		$.bind_props($$props, {
			variant,
			size
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
