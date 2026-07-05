App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><path d="M0 0"></path></svg>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let active = $.prop($$props, "active", 3, false);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	let classes;
	$.template_effect(() => classes = $.set_class(svg, 0, "icon", null, classes, { active: active() }));
	$.append($$anchor, svg);
	return $.pop($$exports);
}
