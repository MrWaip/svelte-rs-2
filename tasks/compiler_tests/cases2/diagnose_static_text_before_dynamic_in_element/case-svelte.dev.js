App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h3>Hello<br/> </h3>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 9]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "x";
	var $$exports = { ...$.legacy_api() };
	var h3 = root();
	var text = $.sibling($.child(h3), 2, true);
	text.nodeValue = "x";
	$.reset(h3);
	$.append($$anchor, h3);
	return $.pop($$exports);
}
