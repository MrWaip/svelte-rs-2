import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <input/>`, 1), App[$.FILENAME], [[7, 1], [8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let people = $.prop($$props, "people", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, people, $.index, ($$anchor, $$item) => {
		let f = () => $.get($$item).name.first;
		f();
		let l = () => $.get($$item).name.last;
		l();
		var fragment_1 = root();
		var input = $.first_child(fragment_1);
		$.remove_input_defaults(input);
		var input_1 = $.sibling(input, 2);
		$.remove_input_defaults(input_1);
		$.bind_value(input, function get() {
			return f();
		}, function set($$value) {
			$.get($$item).name.first = $$value, $.invalidate_inner_signals(() => people());
		});
		$.bind_value(input_1, function get() {
			return l();
		}, function set($$value) {
			$.get($$item).name.last = $$value, $.invalidate_inner_signals(() => people());
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
