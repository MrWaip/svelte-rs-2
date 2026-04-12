import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$anchor) {
	Outer($$anchor, {
		children: ($$anchor, $$slotProps) => Inner($$anchor, { slot: "footer" }),
		$$slots: { default: true }
	});
}
