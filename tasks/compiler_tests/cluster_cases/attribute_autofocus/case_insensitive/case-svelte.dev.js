App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let enabled = true;
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.autofocus(div, enabled);
	$.append($$anchor, div);
	return $.pop($$exports);
}
