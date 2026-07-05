App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><g></g><!></svg>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let content = "<circle cx='5' cy='5' r='5'></circle>";
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	var node = $.sibling($.child(svg));
	$.html(node, () => content, void 0, true);
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
