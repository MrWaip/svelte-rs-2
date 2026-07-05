import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1>t</h1> <img src="x" loading="lazy"/>`, 1), App[$.FILENAME], [[1, 0], [1, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var img = $.sibling($.first_child(fragment), 2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
