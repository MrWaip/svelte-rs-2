import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { content: ($$anchor, $$slotProps) => {
		$.add_svelte_meta(() => Inner($$anchor, { get prop() {
			return value();
		} }), "component", App, 9, 2, { componentTag: "Inner" });
	} } }), "component", App, 7, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
