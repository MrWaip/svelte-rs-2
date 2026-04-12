import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_1 = $.from_html(`<p slot="item"> </p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		var p = root_1();
		const item = $.derived(() => {
			let { text } = $$slotProps.item;
			return { text };
		});
		const index = $.derived_safe_equal(() => $$slotProps.index);
		var text_1 = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text_1, `${$.get(item).text ?? ""} ${$.get(index) ?? ""}`));
		$.append($$anchor, p);
	} } });
}
