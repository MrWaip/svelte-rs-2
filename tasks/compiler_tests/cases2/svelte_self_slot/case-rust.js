import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor) {
	Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		App(node, { slot: "footer" });
		$.append($$anchor, fragment_1);
	} } });
}
