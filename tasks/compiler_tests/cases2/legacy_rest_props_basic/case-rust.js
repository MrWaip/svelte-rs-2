import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click me</button>`);
export default function App($$anchor, $$props) {
	let variant = "filled";
	var button = root();
	$.attribute_effect(button, () => ({
		...$$restProps,
		class: `variant-${variant ?? ""}`
	}));
	$.append($$anchor, button);
}
