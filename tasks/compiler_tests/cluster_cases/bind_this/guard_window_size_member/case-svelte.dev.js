App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = { w: 0 };
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.validate_binding("bind:innerWidth={obj.w}", [], () => obj, () => "w", 4, 15);
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, obj.w));
	$.bind_window_size("innerWidth", function set($$value) {
		obj.w = $$value;
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
