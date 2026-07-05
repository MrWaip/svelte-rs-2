App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><path d="M1"></path></svg><g><path d="M2"></path></g>`, 1), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
], [
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
