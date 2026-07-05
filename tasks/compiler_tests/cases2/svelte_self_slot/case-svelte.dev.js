App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		$.add_svelte_meta(() => App(node, { slot: "footer" }), "component", App, 6, 1, { componentTag: "svelte:self" });
		$.append($$anchor, fragment_1);
	} } }), "component", App, 5, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
