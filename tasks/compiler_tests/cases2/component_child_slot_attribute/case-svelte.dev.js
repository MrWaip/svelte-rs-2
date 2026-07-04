import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		$.add_svelte_meta(() => Inner($$anchor, { slot: "footer" }), "component", App, 7, 1, { componentTag: "Inner" });
	} } }), "component", App, 6, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
