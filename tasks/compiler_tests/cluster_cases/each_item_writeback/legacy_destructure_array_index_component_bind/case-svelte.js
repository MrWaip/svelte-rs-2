import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = () => $.get($$array)[0];
		Child($$anchor, {
			get value() {
				return first();
			},
			set value($$value) {
				$$array[0] = $$value, $.invalidate_inner_signals(() => rows());
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
}
