import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[9, 1]]);
var root_1 = $.add_locations($.from_html(`<button>x</button> <!>`, 1), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let todos = $.tag($.mutable_source([{ done: false }]), "todos");
	function update() {
		$.set(todos, $.get(todos).slice(1));
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(todos), $.index, ($$anchor, todo, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_checked(input, function get() {
			return $.get(todo).done;
		}, function set($$value) {
			$.get(todo).done = $$value, $.invalidate_inner_signals(() => $.get(todos));
		});
		$.append($$anchor, input);
	}), "each", App, 8, 0);
	$.event("click", button, update);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
