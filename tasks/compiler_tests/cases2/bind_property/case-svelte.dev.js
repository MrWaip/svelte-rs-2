App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/> <details></details>`, 1), App[$.FILENAME], [[6, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let indeterminate = $.tag($.state(false), "indeterminate");
	let open = $.tag($.state(true), "open");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	var details = $.sibling(input, 2);
	$.bind_property("indeterminate", "change", input, function set($$value) {
		$.set(indeterminate, $$value);
	}, function get() {
		return $.get(indeterminate);
	});
	$.bind_property("open", "toggle", details, function set($$value) {
		$.set(open, $$value);
	}, function get() {
		return $.get(open);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
