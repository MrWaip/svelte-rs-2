import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		Inner($$anchor, {
			slot: "footer",
			get x() {
				return value();
			}
		});
	} } });
}
