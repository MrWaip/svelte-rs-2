App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const c = "red";
	const w = "bold";
	const s = "16px";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_style(div, "", {}, {
		color: c,
		"font-weight": w,
		"font-size": s
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
