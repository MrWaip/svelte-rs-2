App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var option_content = $.add_locations($.from_html(`<span>A</span>`, 1), App[$.FILENAME], [[2, 19]]);
var root = $.add_locations($.from_html(`<select><option><!></option><option>B</option></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1], [3, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment = option_content();
		$.append(anchor, fragment);
	});
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
