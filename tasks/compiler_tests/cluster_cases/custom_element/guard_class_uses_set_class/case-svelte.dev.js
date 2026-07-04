import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<my-element></my-element>`, 2), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let cls = $.prop($$props, "cls", 8);
	var $$exports = { ...$.legacy_api() };
	var my_element = root();
	$.template_effect(() => $.set_class(my_element, 1, $.clsx(cls())));
	$.append($$anchor, my_element);
	return $.pop($$exports);
}
