App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg></svg><svg></svg>`, 1), App[$.FILENAME], [[1, 0], [1, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
