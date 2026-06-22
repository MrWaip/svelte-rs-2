import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	let a = $.mutable_source([{
		a: { b: "x" },
		key: "b"
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(a), $.index, ($$anchor, $$item, $$index, $$array) => {
		let a = () => $.get($$item).a;
		let key = () => $.get($$item).key;
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => a()[key()], ($$value) => (a()[key()] = $$value, $.invalidate_inner_signals(() => $$array())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
