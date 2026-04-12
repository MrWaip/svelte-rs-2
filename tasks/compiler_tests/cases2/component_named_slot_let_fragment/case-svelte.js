import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		var p = root_2();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, ($.deep_read_state($.get(item)), $.untrack(() => $.get(item).text))));
		$.append($$anchor, p);
	} } });
}
