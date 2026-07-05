App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg viewBox="0 0 24 24" fill="none" width="24" height="24" xmlns="http://www.w3.org/2000/svg"><rect x="3" y="11" width="6" height="10" stroke="#0070f3" stroke-width="2"></rect><rect x="9" y="7" width="6" height="14" stroke="#0070f3" stroke-width="2"></rect><rect x="15" y="3" width="6" height="18" stroke="#0070f3" stroke-width="2"></rect></svg>`), App[$.FILENAME], [[
	5,
	0,
	[
		[6, 1],
		[7, 1],
		[8, 1]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	$.append($$anchor, svg);
	return $.pop($$exports);
}
