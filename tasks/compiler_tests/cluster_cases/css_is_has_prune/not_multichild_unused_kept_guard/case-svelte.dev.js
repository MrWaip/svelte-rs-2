import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<a class="svelte-190fkm3"><b>b</b></a>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 3]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var a = root();
	$.append($$anchor, a);
	return $.pop($$exports);
}
