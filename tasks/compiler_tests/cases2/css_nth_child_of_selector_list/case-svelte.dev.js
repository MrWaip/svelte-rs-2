App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<ul><li class="a svelte-1poaahi">x</li><li class="b">y</li></ul>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 4], [1, 24]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var ul = root();
	$.append($$anchor, ul);
	return $.pop($$exports);
}
