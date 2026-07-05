import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img alt="x"/>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let nw = $.prop($$props, "nw", 12);
	var $$exports = { ...$.legacy_api() };
	var img = root();
	$.bind_property("naturalWidth", "load", img, function set($$value) {
		nw($$value);
	});
	$.append($$anchor, img);
	return $.pop($$exports);
}
