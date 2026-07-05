import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<details><summary>x</summary></details>`), App[$.FILENAME], [[
	4,
	0,
	[[4, 30]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let obj = $.prop($$props, "obj", 12);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var details = root();
	$.bind_property("open", "toggle", details, function set($$value) {
		$$ownership_validator.mutation(null, ["obj", "flag"], obj(obj().flag = $$value, true), 4, 20);
	}, function get() {
		return obj().flag;
	});
	$.append($$anchor, details);
	return $.pop($$exports);
}
