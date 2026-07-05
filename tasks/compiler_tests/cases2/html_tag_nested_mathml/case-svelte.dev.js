App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_mathml(`<math><mn>1</mn> <!></math>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let content = "<mi>x</mi>";
	var $$exports = { ...$.legacy_api() };
	var math = root();
	var node = $.sibling($.child(math), 2);
	$.html(node, () => content, void 0, void 0, true);
	$.reset(math);
	$.append($$anchor, math);
	return $.pop($$exports);
}
