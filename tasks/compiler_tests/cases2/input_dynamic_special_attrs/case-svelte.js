import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <input/> <input type="checkbox"/> <option>picked</option>`, 1);
export default function App($$anchor) {
	let value = "";
	let checked = false;
	let selected = true;
	let disabled = false;
	let readonly = false;
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
}
