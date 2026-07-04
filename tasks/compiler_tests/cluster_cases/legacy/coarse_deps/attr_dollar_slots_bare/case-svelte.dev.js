import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>a</p>`), App[$.FILENAME], [[1, 30]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$slots = $.sanitize_slots($$props);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.set_attribute(p, "hidden", $$slots);
	$.append($$anchor, p);
	return $.pop($$exports);
}
