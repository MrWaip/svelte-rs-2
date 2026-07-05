import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option> </option><option>Two</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	$.template_effect(() => {
		$.set_text(text, ($.deep_read_state(item()), $.untrack(() => item().name)));
		if (option_value !== (option_value = ($.deep_read_state(item()), $.untrack(() => item().key)))) {
			option.value = (option.__value = ($.deep_read_state(item()), $.untrack(() => item().key))) ?? "";
		}
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
