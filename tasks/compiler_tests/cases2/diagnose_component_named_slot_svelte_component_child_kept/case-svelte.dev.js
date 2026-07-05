import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let flag = $.prop($$props, "flag", 8, false);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, { $$slots: { icon: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.component(node, () => flag() ? A : B, ($$anchor, $$component) => {
			$$component($$anchor, { slot: "icon" });
		}), "component", App, 9, 4, { componentTag: "svelte:component" });
		$.append($$anchor, fragment_1);
	} } }), "component", App, 8, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
