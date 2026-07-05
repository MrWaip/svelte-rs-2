App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let w = $.tag($.state(0), "w");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(w)));
	$.bind_window_size("innerWidth", function set($$value) {
		$.set(w, $$value, true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
