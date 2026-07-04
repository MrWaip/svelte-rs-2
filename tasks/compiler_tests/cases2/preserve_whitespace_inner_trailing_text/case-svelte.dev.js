App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`

<div>
	<span>hello</span>

</div>`, 1), App[$.FILENAME], [[
	3,
	0,
	[[4, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
