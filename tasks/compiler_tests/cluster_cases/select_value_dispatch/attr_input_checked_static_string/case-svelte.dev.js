App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let checked = $.tag($.state(false), "checked");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "x";
	$.bind_checked(input, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
