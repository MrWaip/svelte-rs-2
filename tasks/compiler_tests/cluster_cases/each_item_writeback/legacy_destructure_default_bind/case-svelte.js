import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let array = $.prop($$props, "array", 24, () => [{ value: "" }, {}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, array, $.index, ($$anchor, $$item) => {
		let value = $.derived_safe_equal(() => $.fallback($.get($$item).value, "hello"));
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(value), ($$value) => ($.get($$item).value = $$value, $.invalidate_inner_signals(() => array())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
