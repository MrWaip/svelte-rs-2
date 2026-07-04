import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><a></a></svg>`);
export default function App($$anchor, $$props) {
	var svg = root();
	var a = $.child(svg);
	$.reset(svg);
	$.template_effect(() => $.set_xlink_attribute(a, "xlink:href", $$props.href));
	$.append($$anchor, svg);
}
