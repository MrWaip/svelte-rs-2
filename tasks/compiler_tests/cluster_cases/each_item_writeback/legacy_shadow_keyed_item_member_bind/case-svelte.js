import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let a = $.mutable_source([{
		id: 1,
		v: "x"
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(a), (a) => a.id, ($$anchor, a, $$index, $$array) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(a).v, ($$value) => ($.get(a).v = $$value, $.invalidate_inner_signals(() => $$array())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
