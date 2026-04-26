import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click me</button>`);
export default function App($$anchor, $$props) {
	let variant = $.prop($$props, "variant", 8);
	var button = root();
	$.attribute_effect(button, () => ({
		...$$props,
		class: `variant-${variant() ?? ""} ${$$props.class ?? "" ?? ""}`
	}));
	$.append($$anchor, button);
}
