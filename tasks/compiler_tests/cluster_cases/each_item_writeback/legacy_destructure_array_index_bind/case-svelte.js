import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <span> </span>`, 1);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let first = () => $.get($$array)[0];
		let second = () => $.get($$array)[1];
		var fragment_1 = root();
		var input = $.first_child(fragment_1);
		$.remove_input_defaults(input);
		var span = $.sibling(input, 2);
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, second()));
		$.bind_value(input, first, ($$value) => ($$array[0] = $$value, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
