import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let placeholder = $.prop($$props, "placeholder", 8);
	let fieldData = $.prop($$props, "fieldData", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(() => $.set_value(input, ($.deep_read_state(placeholder()), $.deep_read_state(fieldData()), $.untrack(() => placeholder() || fieldData().label))));
	$.append($$anchor, input);
	return $.pop($$exports);
}
