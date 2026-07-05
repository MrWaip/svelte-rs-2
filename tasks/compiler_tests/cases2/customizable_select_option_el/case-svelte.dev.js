App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var option_content = $.add_locations($.from_html(`<div>text</div>`, 1), App[$.FILENAME], [[5, 9]]);
var root = $.add_locations($.from_html(`<select><option><!></option></select>`), App[$.FILENAME], [[
	4,
	0,
	[[5, 1]]
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
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
