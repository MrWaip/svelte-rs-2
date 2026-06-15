import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
var root = $.from_html(`<button>x</button> <!>`, 1);
export default function App($$anchor) {
	let todos = $.mutable_source([{ done: false }]);
	function update() {
		$.set(todos, $.get(todos).slice(1));
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 1, () => $.get(todos), $.index, ($$anchor, todo, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_checked(input, () => $.get(todo).done, ($$value) => ($.get(todo).done = $$value, $.invalidate_inner_signals(() => $.get(todos))));
		$.append($$anchor, input);
	});
	$.event("click", button, update);
	$.append($$anchor, fragment);
}
