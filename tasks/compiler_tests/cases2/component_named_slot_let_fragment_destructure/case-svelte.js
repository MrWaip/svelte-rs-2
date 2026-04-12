import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived(() => {
			let { text } = $$slotProps.item;
			return { text };
		});
		var p = root_2();
		var text_1 = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text_1, $.get(item).text));
		$.append($$anchor, p);
	} } });
}
