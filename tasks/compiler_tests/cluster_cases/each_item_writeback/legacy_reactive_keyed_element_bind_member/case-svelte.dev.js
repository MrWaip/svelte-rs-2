import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let todos = $.tag($.mutable_source([{
		id: 1,
		done: false
	}]), "todos");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(todos), (todo) => todo.id, ($$anchor, todo, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_checked(input, function get() {
			return $.get(todo).done;
		}, function set($$value) {
			$.get(todo).done = $$value, $.invalidate_inner_signals(() => $.get(todos));
		});
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
