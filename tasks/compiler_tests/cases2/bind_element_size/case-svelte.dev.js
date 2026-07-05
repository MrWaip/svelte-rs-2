App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <div></div>`, 1), App[$.FILENAME], [[6, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let w = $.tag($.state(0), "w");
	let h = $.tag($.state(0), "h");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	$.bind_element_size(div, "clientWidth", function set($$value) {
		$.set(w, $$value);
	});
	$.bind_element_size(div, "clientHeight", function set($$value) {
		$.set(h, $$value);
	});
	$.bind_element_size(div_1, "offsetWidth", function set($$value) {
		$.set(w, $$value);
	});
	$.bind_element_size(div_1, "offsetHeight", function set($$value) {
		$.set(h, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
