import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { row: ($$anchor, $$slotProps) => {
		const row = $.derived_safe_equal(() => $$slotProps.row);
		const v = $.derived_safe_equal(() => ($.deep_read_state($.get(row)), $.untrack(() => $.get(row).value * 2)));
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, p);
	} } });
}
