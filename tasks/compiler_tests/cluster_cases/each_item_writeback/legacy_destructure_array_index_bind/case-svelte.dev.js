import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <span> </span>`, 1), App[$.FILENAME], [[7, 1], [8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let first = () => $.get($$array)[0];
		first();
		let second = () => $.get($$array)[1];
		second();
		var fragment_1 = root();
		var input = $.first_child(fragment_1);
		$.remove_input_defaults(input);
		var span = $.sibling(input, 2);
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, second()));
		$.bind_value(input, function get() {
			return first();
		}, function set($$value) {
			$$array[0] = $$value, $.invalidate_inner_signals(() => rows());
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
