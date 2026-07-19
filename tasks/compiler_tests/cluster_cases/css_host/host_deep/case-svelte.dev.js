import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1>h</h1><div><span class="svelte-1pgigeb">s</span></div>`, 1), App[$.FILENAME], [[1, 0], [
	1,
	10,
	[[1, 15]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
