App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let num = 42;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "42";
	$.append($$anchor, p);
	return $.pop($$exports);
}
