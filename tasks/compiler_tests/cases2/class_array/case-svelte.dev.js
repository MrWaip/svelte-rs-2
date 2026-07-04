App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let active = false;
	const base = "btn";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_class(div, 1, $.clsx([
		base,
		active && "active",
		"extra"
	]));
	$.append($$anchor, div);
	return $.pop($$exports);
}
