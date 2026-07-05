App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let h = $.prop($$props, "h", 15, 0);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_element_size(div, "clientHeight", function set($$value) {
		h($$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
