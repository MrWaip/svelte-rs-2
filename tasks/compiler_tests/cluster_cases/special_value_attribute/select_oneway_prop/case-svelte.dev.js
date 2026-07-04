import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>a</option><option>b</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(() => {
		if (select_value !== (select_value = foo())) {
			select.value = (select.__value = foo()) ?? "", $.select_option(select, foo());
		}
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
