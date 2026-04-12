import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click me</button>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["variant"]);
	let variant = $.prop($$props, "variant", 8, "filled");
	var button = root();
	$.attribute_effect(button, () => ({
		...$$restProps,
		class: `variant-${variant() ?? ""}`
	}));
	$.append($$anchor, button);
}
