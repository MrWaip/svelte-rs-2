import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		const index = $.derived_safe_equal(() => $$slotProps.index);
		$.add_svelte_meta(() => Child($$anchor, {
			slot: "item",
			get item() {
				return $.get(item);
			},
			get index() {
				return $.get(index);
			}
		}), "component", App, 7, 1, { componentTag: "Child" });
	} } }), "component", App, 6, 0, { componentTag: "Parent" });
	return $.pop($$exports);
}
