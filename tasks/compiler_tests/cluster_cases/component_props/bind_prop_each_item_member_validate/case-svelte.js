import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $$props.rows, $.index, ($$anchor, row, i) => {
		Child($$anchor, {
			get value() {
				return $.get(row).name;
			},
			set value($$value) {
				$.get(row).name = $$value;
			}
		});
	});
	$.append($$anchor, fragment);
	$.pop();
}
