App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<tr><td> </td></tr> <tr><td> </td></tr>`, 1), App[$.FILENAME], [[
	7,
	2,
	[[7, 6]]
], [
	8,
	2,
	[[8, 6]]
]]);
var root_1 = $.add_locations($.from_html(`<table><tbody></tbody></table>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 7]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var table = root_1();
	var tbody = $.child(table);
	$.add_svelte_meta(() => $.each(tbody, 21, () => $$props.rows, $.index, ($$anchor, r) => {
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
	}), "each", App, 6, 1);
	$.reset(tbody);
	$.reset(table);
	$.append($$anchor, table);
	return $.pop($$exports);
}
