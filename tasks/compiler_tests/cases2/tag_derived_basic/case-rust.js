App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let doubled = $.tag($.derived(() => count * 2), "doubled");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = $.get(doubled);
	$.append($$anchor, p);
	return $.pop($$exports);
}
