import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Cell from "./Cell.svelte";
var root_1 = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, row) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		{
			$.css_props(node_1, () => ({ "--tone": `prefix-${($.get(row), $.untrack(() => $.get(row).kind)) ?? ""}-suffix` }));
			Cell(node_1.lastChild, {});
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
