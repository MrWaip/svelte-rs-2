import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	let refs = $.mutable_source({});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		var div = root_1();
		$.bind_this(div, ($$value, row) => $.mutate(refs, $.get(refs)[row.key] = $$value), (row) => $.get(refs)?.[row.key], () => [$.get(row)]);
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
