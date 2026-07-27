import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item, $$index) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = () => $.get($$array)[0];
		first();
		var input = root();
		$.remove_input_defaults(input);
		input.value = input.__value = "a";
		$.bind_group(binding_group, [$$index], input, function get() {
			return first();
		}, function set($$value) {
			$$array[0] = $$value, $.invalidate_inner_signals(() => rows());
		});
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
