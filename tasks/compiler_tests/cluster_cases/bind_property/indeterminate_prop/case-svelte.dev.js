import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let indeterminate = $.prop($$props, "indeterminate", 12);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.bind_property("indeterminate", "change", input, function set($$value) {
		indeterminate($$value);
	}, function get() {
		return indeterminate();
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
