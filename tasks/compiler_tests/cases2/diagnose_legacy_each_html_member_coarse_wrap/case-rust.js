import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		var p = root_1();
		$.html(p, () => ($.get(row), $.untrack(() => $.get(row).content)), true);
		$.reset(p);
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
