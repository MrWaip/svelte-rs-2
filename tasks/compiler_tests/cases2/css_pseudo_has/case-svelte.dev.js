App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="card svelte-mv1sf"><span class="inside svelte-mv1sf">inside</span></div> <span class="inside">outside</span>`, 1), App[$.FILENAME], [[
	5,
	0,
	[[6, 4]]
], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
