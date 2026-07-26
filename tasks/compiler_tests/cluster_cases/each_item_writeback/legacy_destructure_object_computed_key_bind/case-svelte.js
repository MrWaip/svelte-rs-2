import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	let key = $.prop($$props, "key", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		let value = () => $.get($$item)[key()];
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, value, ($$value) => ($.get($$item)[key] = $$value, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
