import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
var root_2 = $.from_html(`<span slot="label"> </span>`);
var root_1 = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, row) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		{
			$.css_props(node_1, () => ({ "--tone": "red" }));
			Row(node_1.lastChild, { $$slots: { label: ($$anchor, $$slotProps) => {
				var span = root_2();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, ($.get(row), $.untrack(() => $.get(row).title))));
				$.append($$anchor, span);
			} } });
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
