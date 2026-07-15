App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<section>Раздел <span>Внутри <em>текст</em></span></section>`), App[$.FILENAME], [[
	1,
	0,
	[[
		1,
		16,
		[[1, 29]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var section = root();
	$.append($$anchor, section);
	return $.pop($$exports);
}
