import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
let label = "hi";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "hi";
	$.append($$anchor, p);
	return $.pop($$exports);
}
