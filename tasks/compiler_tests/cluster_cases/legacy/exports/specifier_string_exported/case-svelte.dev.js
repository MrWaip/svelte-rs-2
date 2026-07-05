import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = 1;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "1";
	$.append($$anchor, p);
	return $.pop($$exports);
}
