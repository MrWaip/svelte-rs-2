import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let list = $.mutable_source([{}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(list), $.index, ($$anchor, item, $$index) => {
		Child($$anchor, {
			get value() {
				return $.get(list)[$$index];
			},
			set value($$value) {
				$.get(list)[$$index] = $$value, $.invalidate_inner_signals(() => $.get(list));
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
}
