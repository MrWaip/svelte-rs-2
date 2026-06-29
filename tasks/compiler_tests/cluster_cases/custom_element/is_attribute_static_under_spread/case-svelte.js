import * as $ from "svelte/internal/client";
var root = $.from_html(`<button is="x-button">x</button>`, 2);
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var button = root();
	$.attribute_effect(button, () => ({ ...props }));
	$.append($$anchor, button);
}
