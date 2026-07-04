App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><a></a></svg>`), App[$.FILENAME], [[
	5,
	0,
	[[7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	var a = $.child(svg);
	$.reset(svg);
	$.template_effect(() => $.set_xlink_attribute(a, "xlink:href", $$props.href, true));
	$.append($$anchor, svg);
	return $.pop($$exports);
}
