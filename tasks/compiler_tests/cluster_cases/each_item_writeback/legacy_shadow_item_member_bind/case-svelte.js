import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	let rows = $.mutable_source([{ v: 1 }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(rows), $.index, ($$anchor, rows, $$index, $$array) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(rows).v, ($$value) => ($.get(rows).v = $$value, $.invalidate_inner_signals(() => $$array())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
