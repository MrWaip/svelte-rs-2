import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$anchor) {
	Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		Inner($$anchor, { slot: "footer" });
	} } });
}
