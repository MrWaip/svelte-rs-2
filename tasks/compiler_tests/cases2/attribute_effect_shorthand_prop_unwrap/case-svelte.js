import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let width = $.prop($$props, "width", 3, 1), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"width",
		"alt"
	]);
	var div = root();
	$.attribute_effect(div, () => ({
		width: width(),
		alt: $$props.alt,
		...rest
	}));
	$.append($$anchor, div);
}
