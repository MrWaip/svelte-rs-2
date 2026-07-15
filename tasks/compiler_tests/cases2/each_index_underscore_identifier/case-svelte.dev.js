App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rows = [];
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 19, () => rows, (row) => row.id, ($$anchor, row) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(row).name));
		$.append($$anchor, p);
	}), "each", App, 5, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 17, () => rows, $.index, ($$anchor, row, i_dx) => {
		var span = root_1();
		var text_1 = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text_1, `${$.get(row).name ?? ""}${i_dx}`));
		$.append($$anchor, span);
	}), "each", App, 9, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
