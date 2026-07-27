import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span> <input/> <input/>`, 1);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item)));
		var $$array_1 = $.derived(() => $.to_array($.get($$array).slice(2)));
		let first = () => $.get($$array)[0];
		let second = () => $.get($$array)[1];
		let third = () => $.get($$array_1)[0];
		let length = () => $.get($$array_1).slice(1).length;
		var fragment_1 = root();
		var span = $.first_child(fragment_1);
		var text = $.child(span);
		$.reset(span);
		var input = $.sibling(span, 2);
		$.remove_input_defaults(input);
		var input_1 = $.sibling(input, 2);
		$.remove_input_defaults(input_1);
		$.template_effect(() => $.set_text(text, `${first() ?? ""}${second() ?? ""}`));
		$.bind_value(input, third, ($$value) => ($$array_1[0] = $$value, $.invalidate_inner_signals(() => rows())));
		$.bind_value(input_1, length, ($$value) => ($$array_1.slice(1).length = $$value, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
