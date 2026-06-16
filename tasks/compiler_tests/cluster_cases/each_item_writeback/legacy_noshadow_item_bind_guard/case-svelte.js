import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	let list = $.mutable_source(["Hello"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(list), $.index, ($$anchor, item, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(list)[$$index], ($$value) => ($.get(list)[$$index] = $$value, $.invalidate_inner_signals(() => $.get(list))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
