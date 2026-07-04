import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<my-el></my-el>`, 2), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var my_el = root();
	$.set_class(my_el, 1, "foo");
	$.append($$anchor, my_el);
	return $.pop($$exports);
}
