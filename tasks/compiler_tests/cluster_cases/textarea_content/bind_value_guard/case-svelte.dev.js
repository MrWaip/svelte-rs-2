import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<textarea></textarea>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.bind_value(textarea, function get() {
		return foo();
	}, function set($$value) {
		foo($$value);
	});
	$.append($$anchor, textarea);
	return $.pop($$exports);
}
