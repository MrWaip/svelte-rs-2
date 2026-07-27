import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let component = $.prop($$props, "component", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		const item = $.derived_safe_equal(() => $$slotProps.item);
		const index = $.derived_safe_equal(() => $$slotProps.index);
		$.add_svelte_meta(() => $.component(node, component, ($$anchor, $$component) => {
			$$component($$anchor, {
				slot: "item",
				get item() {
					return $.get(item);
				},
				get index() {
					return $.get(index);
				}
			});
		}), "component", App, 7, 1, { componentTag: "svelte:component" });
		$.append($$anchor, fragment_1);
	} } }), "component", App, 6, 0, { componentTag: "Parent" });
	return $.pop($$exports);
}
