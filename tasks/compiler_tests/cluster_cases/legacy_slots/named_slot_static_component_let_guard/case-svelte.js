import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		const index = $.derived_safe_equal(() => $$slotProps.index);
		Child($$anchor, {
			slot: "item",
			get item() {
				return $.get(item);
			},
			get index() {
				return $.get(index);
			}
		});
	} } });
}
