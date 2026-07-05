App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option></option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 1;
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.textContent = "1";
	option.__value = value;
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
