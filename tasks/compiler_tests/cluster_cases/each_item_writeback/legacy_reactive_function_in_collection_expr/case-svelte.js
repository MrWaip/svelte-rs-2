import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	let items = $.mutable_source([{ value: "x" }]);
	function keep(it) {
		return true;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.get(items), $.untrack(() => $.get(items).filter(keep))), $.index, ($$anchor, item, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(item).value, ($$value) => ($.get(item).value = $$value, $.invalidate_inner_signals(() => $.get(items))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
