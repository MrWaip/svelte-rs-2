import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button> <div></div>`, 1), App[$.FILENAME], [[4, 0], [5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let col = $.tag($.mutable_source("red"), "col");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: $.get(col) }));
	$.event("click", button, function click() {
		return $.set(col, "blue");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
