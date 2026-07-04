App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let box_size = $.tag($.state(void 0), "box_size");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_resize_observer(div, "borderBoxSize", function set($$value) {
		$.set(box_size, $$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
