import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item, $$index) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = () => $.get($$array)[0];
		var input = root();
		$.remove_input_defaults(input);
		input.value = input.__value = "a";
		$.bind_group(binding_group, [$$index], input, first, ($$value) => ($$array[0] = $$value, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
