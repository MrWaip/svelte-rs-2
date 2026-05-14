import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	Outer($$anchor, { $$slots: { content: ($$anchor, $$slotProps) => {
		Inner($$anchor, { get prop() {
			return value();
		} });
	} } });
}
