import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"width",
	"alt"
]);
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let width = $.prop($$props, "width", 3, 1), rest = $.rest_props($$props, rest_excludes);
	var div = root();
	$.attribute_effect(div, () => ({
		width: width(),
		alt: $$props.alt,
		...rest
	}));
	$.append($$anchor, div);
}
