import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <input/>`, 1);
export default function App($$anchor, $$props) {
	let people = $.prop($$props, "people", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, people, $.index, ($$anchor, $$item) => {
		let f = () => $.get($$item).name.first;
		let l = () => $.get($$item).name.last;
		var fragment_1 = root();
		var input = $.first_child(fragment_1);
		$.remove_input_defaults(input);
		var input_1 = $.sibling(input, 2);
		$.remove_input_defaults(input_1);
		$.bind_value(input, f, ($$value) => ($.get($$item).name.first = $$value, $.invalidate_inner_signals(() => people())));
		$.bind_value(input_1, l, ($$value) => ($.get($$item).name.last = $$value, $.invalidate_inner_signals(() => people())));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
