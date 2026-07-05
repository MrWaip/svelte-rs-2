import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let value = $.prop($$props, "value", 12, undefined);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	$.bind_group(binding_group, [], input, function get() {
		return value();
	}, function set($$value) {
		value($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
