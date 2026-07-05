App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <div></div>`, 1), App[$.FILENAME], [[6, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rect = $.tag($.state(void 0), "rect");
	let box_size = $.tag($.state(void 0), "box_size");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	$.bind_resize_observer(div, "contentRect", function set($$value) {
		$.set(rect, $$value);
	});
	$.bind_resize_observer(div_1, "contentBoxSize", function set($$value) {
		$.set(box_size, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
