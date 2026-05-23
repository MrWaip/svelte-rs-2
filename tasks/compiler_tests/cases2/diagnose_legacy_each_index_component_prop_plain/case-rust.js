import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, row, index) => {
		Row($$anchor, {
			icon: index + 1,
			get title() {
				return $.get(row), $.untrack(() => $.get(row).title);
			}
		});
	});
	$.append($$anchor, fragment);
}
