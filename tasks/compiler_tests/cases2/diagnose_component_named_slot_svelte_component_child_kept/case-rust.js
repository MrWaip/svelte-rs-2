import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$anchor, $$props) {
	let flag = $.prop($$props, "flag", 8, false);
	Inner($$anchor, { $$slots: { icon: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		$.component(node, () => flag() ? A : B, ($$anchor, $$component) => {
			$$component($$anchor, { slot: "icon" });
		});
		$.append($$anchor, fragment_1);
	} } });
}
