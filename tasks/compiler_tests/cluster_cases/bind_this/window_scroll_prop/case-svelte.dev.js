import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let scrollY = $.prop($$props, "scrollY", 12);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, scrollY()));
	$.bind_window_scroll("y", function get() {
		return scrollY();
	}, function set($$value) {
		scrollY($$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
