import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	let items = $.mutable_source(["a", "b"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 3, () => $.get(items), (item) => item, ($$anchor, item, idx) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(items)[$.get(idx)], ($$value) => ($.get(items)[$.get(idx)] = $$value, $.invalidate_inner_signals(() => $.get(items))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
