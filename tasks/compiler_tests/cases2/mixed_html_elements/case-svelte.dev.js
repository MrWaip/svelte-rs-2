App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <textarea></textarea> <area/> <br/> <a></a>`, 1), App[$.FILENAME], [
	[1, 0],
	[2, 0],
	[3, 0],
	[4, 0],
	[5, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(8);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
