import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
var root = $.from_html(`<div slot="item"> </div>`);
export default function App($$anchor) {
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	} } });
}
