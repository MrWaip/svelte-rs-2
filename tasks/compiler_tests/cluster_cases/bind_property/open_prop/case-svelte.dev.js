import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<details><summary>x</summary></details>`), App[$.FILENAME], [[
	4,
	0,
	[[4, 29]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let visible = $.prop($$props, "visible", 12);
	var $$exports = { ...$.legacy_api() };
	var details = root();
	$.bind_property("open", "toggle", details, function set($$value) {
		visible($$value);
	}, function get() {
		return visible();
	});
	$.append($$anchor, details);
	return $.pop($$exports);
}
