App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const w = "12" + "px";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_style(div, "", {}, { width: w });
	$.append($$anchor, div);
	return $.pop($$exports);
}
