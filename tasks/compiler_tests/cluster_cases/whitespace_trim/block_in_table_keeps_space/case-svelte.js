import * as $ from "svelte/internal/client";
var root = $.from_html(`<tr><td> </td></tr> <tr><td> </td></tr>`, 1);
var root_1 = $.from_html(`<table><tbody></tbody></table>`);
export default function App($$anchor, $$props) {
	var table = root_1();
	var tbody = $.child(table);
	$.each(tbody, 21, () => $$props.rows, $.index, ($$anchor, r) => {
		var fragment = root();
		var tr = $.first_child(fragment);
		var td = $.child(tr);
		var text = $.child(td, true);
		$.reset(td);
		$.reset(tr);
		var tr_1 = $.sibling(tr, 2);
		var td_1 = $.child(tr_1);
		var text_1 = $.child(td_1, true);
		$.reset(td_1);
		$.reset(tr_1);
		$.template_effect(() => {
			$.set_text(text, $.get(r));
			$.set_text(text_1, $.get(r));
		});
		$.append($$anchor, fragment);
	});
	$.reset(tbody);
	$.reset(table);
	$.append($$anchor, table);
}
