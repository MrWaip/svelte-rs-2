App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>--Please choose--</option><option>Dog</option><option>Cat</option></select>`), App[$.FILENAME], [[
	1,
	0,
	[
		[2, 1],
		[3, 1],
		[4, 1]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "dog";
	var option_2 = $.sibling(option_1);
	option_2.value = option_2.__value = "cat";
	$.reset(select);
	select.value = select.__value = "dog";
	$.append($$anchor, select);
	return $.pop($$exports);
}
