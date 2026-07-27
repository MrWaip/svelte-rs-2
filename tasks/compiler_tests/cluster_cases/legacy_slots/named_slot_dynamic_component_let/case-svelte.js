import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
export default function App($$anchor, $$props) {
	let component = $.prop($$props, "component", 8);
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		const item = $.derived_safe_equal(() => $$slotProps.item);
		const index = $.derived_safe_equal(() => $$slotProps.index);
		$.component(node, component, ($$anchor, $$component) => {
			$$component($$anchor, {
				slot: "item",
				get item() {
					return $.get(item);
				},
				get index() {
					return $.get(index);
				}
			});
		});
		$.append($$anchor, fragment_1);
	} } });
}
