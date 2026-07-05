import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><my-element></my-element></div>`, 2), App[$.FILENAME], [[
	1,
	0,
	[[1, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var my_element = $.child(div);
	$.set_class(my_element, 1, "red svelte-p153w3");
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
