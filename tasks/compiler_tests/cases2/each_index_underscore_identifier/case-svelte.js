import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<span> </span>`);
var root_2 = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let rows = [];
	var fragment = root_2();
	var node = $.first_child(fragment);
	$.each(node, 19, () => rows, (row) => row.id, ($$anchor, row) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(row).name));
		$.append($$anchor, p);
	});
	var node_1 = $.sibling(node, 2);
	$.each(node_1, 17, () => rows, $.index, ($$anchor, row, i_dx) => {
		var span = root_1();
		var text_1 = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text_1, `${$.get(row).name ?? ""}${i_dx}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
