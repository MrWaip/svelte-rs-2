import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = $.derived_safe_equal(() => $.fallback($.get($$array)[0], "x"));
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(first), ($$value) => ($$array[0] = $$value, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
