import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let items = $.mutable_source(["a", "b"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(items), $.index, ($$anchor, item, idx) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(items)[idx], ($$value) => ($.get(items)[idx] = $$value, $.invalidate_inner_signals(() => $.get(items))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
