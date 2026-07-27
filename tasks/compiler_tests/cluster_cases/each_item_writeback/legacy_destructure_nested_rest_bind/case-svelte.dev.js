import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span> <input/> <input/>`, 1), App[$.FILENAME], [
	[7, 1],
	[8, 1],
	[9, 1]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item)));
		var $$array_1 = $.derived(() => $.to_array($.get($$array).slice(2)));
		let first = () => $.get($$array)[0];
		first();
		let second = () => $.get($$array)[1];
		second();
		let third = () => $.get($$array_1)[0];
		third();
		let length = () => $.get($$array_1).slice(1).length;
		length();
		var fragment_1 = root();
		var span = $.first_child(fragment_1);
		var text = $.child(span);
		$.reset(span);
		var input = $.sibling(span, 2);
		$.remove_input_defaults(input);
		var input_1 = $.sibling(input, 2);
		$.remove_input_defaults(input_1);
		$.template_effect(() => $.set_text(text, `${first() ?? ""}${second() ?? ""}`));
		$.bind_value(input, function get() {
			return third();
		}, function set($$value) {
			$$array_1[0] = $$value, $.invalidate_inner_signals(() => rows());
		});
		$.bind_value(input_1, function get() {
			return length();
		}, function set($$value) {
			$$array_1.slice(1).length = $$value, $.invalidate_inner_signals(() => rows());
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
