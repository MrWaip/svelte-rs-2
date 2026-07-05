import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source([{
		id: 1,
		v: "x"
	}]), "a");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(a), (a) => a.id, ($$anchor, a, $$index, $$array) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(a).v;
		}, function set($$value) {
			$.get(a).v = $$value, $.invalidate_inner_signals(() => $$array());
		});
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
