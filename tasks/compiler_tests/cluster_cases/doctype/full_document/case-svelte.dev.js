App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!doctype html=""/> <html lang="en"><head><meta charset="utf-8"/> <title>Svelte App</title></head> <body><div>Hello World</div></body></html>`, 1), App[$.FILENAME], [[1, 0], [
	2,
	0,
	[[
		3,
		1,
		[[4, 2], [5, 2]]
	], [
		7,
		1,
		[[8, 2]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
