import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let col = $.prop($$props, "col", 8);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: col() }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
