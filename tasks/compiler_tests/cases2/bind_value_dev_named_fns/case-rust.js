App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let state = $.tag($.state(""), "state");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
