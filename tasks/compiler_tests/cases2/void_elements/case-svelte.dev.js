App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="text"/> <br/> <img src="test.png"/> <hr/>`, 1), App[$.FILENAME], [
	[1, 0],
	[2, 0],
	[3, 0],
	[4, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
