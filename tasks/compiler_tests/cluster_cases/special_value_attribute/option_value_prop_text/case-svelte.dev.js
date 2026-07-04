import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
	var option = root();
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.template_effect(() => {
		$.set_text(text, foo());
		if (option_value !== (option_value = foo())) {
			option.value = (option.__value = foo()) ?? "";
		}
	});
	$.append($$anchor, option);
	return $.pop($$exports);
}
