import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>a</option><option>b</option></select>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 1], [8, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	let items = $.prop($$props, "items", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var select = root();
	var option = $.child(select);
	var option_value = {};
	var option_1 = $.sibling(option);
	var option_1_value = {};
	$.reset(select);
	var select_value;
	$.init_select(select);
	$.template_effect(() => {
		if (option_value !== (option_value = ($.deep_read_state(items()), $.untrack(() => items()[0])))) {
			option.value = (option.__value = ($.deep_read_state(items()), $.untrack(() => items()[0]))) ?? "";
		}
		if (option_1_value !== (option_1_value = ($.deep_read_state(items()), $.untrack(() => items()[1])))) {
			option_1.value = (option_1.__value = ($.deep_read_state(items()), $.untrack(() => items()[1]))) ?? "";
		}
		if (select_value !== (select_value = foo())) {
			select.value = (select.__value = foo()) ?? "", $.select_option(select, foo());
		}
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
