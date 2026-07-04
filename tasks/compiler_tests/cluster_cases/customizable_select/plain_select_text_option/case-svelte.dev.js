App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>A</option><option>B</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag($.state("a"), "v");
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.bind_select_value(select, function get() {
		return $.get(v);
	}, function set($$value) {
		$.set(v, $$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
