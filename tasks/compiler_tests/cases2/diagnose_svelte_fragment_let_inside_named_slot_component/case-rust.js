import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
import Leaf from "./Leaf.svelte";
export default function App($$anchor) {
	Outer($$anchor, { $$slots: { action: ($$anchor, $$slotProps) => {
		Inner($$anchor, {
			slot: "action",
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				const y = $.derived_safe_equal(() => $$slotProps.y);
				Leaf($$anchor, { get value() {
					return $.get(y);
				} });
			} }
		});
	} } });
}
