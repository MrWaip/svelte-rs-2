import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span> </span></div>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, div);
	return $.pop($$exports);
}
