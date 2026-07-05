import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
import Leaf from "./Leaf.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { action: ($$anchor, $$slotProps) => {
		$.add_svelte_meta(() => Inner($$anchor, {
			slot: "action",
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				const y = $.derived_safe_equal(() => $$slotProps.y);
				$.add_svelte_meta(() => Leaf($$anchor, { get value() {
					return $.get(y);
				} }), "component", App, 10, 12, { componentTag: "Leaf" });
			} }
		}), "component", App, 8, 4, { componentTag: "Inner" });
	} } }), "component", App, 7, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
