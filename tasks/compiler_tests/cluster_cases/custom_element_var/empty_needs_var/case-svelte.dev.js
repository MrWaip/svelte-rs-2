App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <my-thing></my-thing>`, 3), App[$.FILENAME], [[1, 0], [2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var my_thing = $.sibling($.first_child(fragment), 2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
