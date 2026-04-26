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
	$.push($$props, false);
	let variant = $.prop($$props, "variant", 8);
	$.init();
	var button = root();
	$.attribute_effect(button, () => ({
		...$$sanitized_props,
		class: `variant-${variant() ?? ""} ${$$sanitized_props.class ?? "" ?? ""}`
	}));
	$.append($$anchor, button);
	$.pop();
}
