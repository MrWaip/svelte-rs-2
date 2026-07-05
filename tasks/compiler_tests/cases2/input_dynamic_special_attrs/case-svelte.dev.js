App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <input/> <input type="checkbox"/> <option>picked</option>`, 1), App[$.FILENAME], [
	[9, 0],
	[10, 0],
	[11, 0],
	[12, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "";
	let checked = false;
	let selected = true;
	let disabled = false;
	let readonly = false;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	$.set_value(input, value);
	input.disabled = disabled;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	$.set_value(input_1, value);
	input_1.readOnly = readonly;
	var input_2 = $.sibling(input_1, 2);
	$.remove_input_defaults(input_2);
	$.set_checked(input_2, checked);
	var option = $.sibling(input_2, 2);
	$.set_selected(option, selected);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
