import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
export default function App($$anchor) {
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, text);
	} } });
}
