import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	let todos = $.mutable_source([{ done: false }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(todos), $.index, ($$anchor, todo, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_checked(input, () => $.get(todo).done, ($$value) => ($.get(todo).done = $$value, $.invalidate_inner_signals(() => $.get(todos))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
