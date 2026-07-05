import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<label>a <input type="checkbox"/></label>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 9]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let test = $.prop($$props, "test", 28, () => []);
	var $$exports = { ...$.legacy_api() };
	var label = root();
	var input = $.sibling($.child(label));
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	$.reset(label);
	$.bind_group(binding_group, [], input, function get() {
		return test();
	}, function set($$value) {
		test($$value);
	});
	$.append($$anchor, label);
	return $.pop($$exports);
}
