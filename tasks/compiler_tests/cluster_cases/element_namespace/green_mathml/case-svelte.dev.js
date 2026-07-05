App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_mathml(`<math><mi>x</mi></math>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 6]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var math = root();
	$.append($$anchor, math);
	return $.pop($$exports);
}
