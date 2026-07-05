import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
var root = $.add_locations($.from_html(`<span slot="label"> </span>`), App[$.FILENAME], [[9, 8]]);
var root_1 = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, row) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		{
			$.css_props(node_1, () => ({ "--tone": "red" }));
			Row(node_1.lastChild, { $$slots: { label: ($$anchor, $$slotProps) => {
				var span = root();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, ($.get(row), $.untrack(() => $.get(row).title))));
				$.append($$anchor, span);
			} } });
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
