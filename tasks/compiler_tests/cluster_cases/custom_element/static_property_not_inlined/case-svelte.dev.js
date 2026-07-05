import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<my-element></my-element>`, 2), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var my_element = root();
	$.set_custom_element_data(my_element, "text", "!");
	$.append($$anchor, my_element);
	return $.pop($$exports);
}
