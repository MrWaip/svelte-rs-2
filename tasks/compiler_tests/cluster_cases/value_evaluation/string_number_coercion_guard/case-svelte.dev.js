App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let d = 2;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "3 3 0 5 ab true";
	$.append($$anchor, p);
	return $.pop($$exports);
}
