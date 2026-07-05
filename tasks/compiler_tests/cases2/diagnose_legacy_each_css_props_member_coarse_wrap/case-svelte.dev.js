import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Cell from "./Cell.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, row) => {
		var fragment_1 = root();
		var node_1 = $.first_child(fragment_1);
		{
			$.css_props(node_1, () => ({ "--tone": ($.get(row), $.untrack(() => $.get(row).muted ? undefined : "accent")) }));
			Cell(node_1.lastChild, {});
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
