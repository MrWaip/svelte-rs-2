App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let group = $.tag($.state($.proxy([])), "group");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "x";
	$.bind_group(binding_group, [], input, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
