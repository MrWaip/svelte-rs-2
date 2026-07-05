App[$.FILENAME] = "(unknown)";
import { writable } from "svelte/store";
import * as $ from "svelte/internal/client";
const theme = writable("light");
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
	return $.pop($$exports);
}
