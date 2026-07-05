App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { foo } from "./foo";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	// this comment should move
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
	return $.pop($$exports);
}
