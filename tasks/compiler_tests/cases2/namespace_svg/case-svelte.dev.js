App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<rect width="100" height="100"></rect>`), App[$.FILENAME], [[3, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var rect = root();
	$.append($$anchor, rect);
	return $.pop($$exports);
}
