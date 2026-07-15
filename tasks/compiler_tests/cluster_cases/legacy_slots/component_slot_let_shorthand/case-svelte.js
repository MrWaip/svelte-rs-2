import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		Child($$anchor, {
			slot: "item",
			onclick: () => $.get(item),
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text("asd");
				$.append($$anchor, text);
			} }
		});
	} } });
}
