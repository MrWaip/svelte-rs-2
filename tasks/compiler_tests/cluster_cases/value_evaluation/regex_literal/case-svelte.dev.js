App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const r = /ab/;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "/ab/ object x/ab/";
	$.append($$anchor, p);
	return $.pop($$exports);
}
