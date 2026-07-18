import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<x class="svelte-p1xjaf"><y class="svelte-p1xjaf">y</y></x>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 3]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var x = root();
	$.append($$anchor, x);
	return $.pop($$exports);
}
