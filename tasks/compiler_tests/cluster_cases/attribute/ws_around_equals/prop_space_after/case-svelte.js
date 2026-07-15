import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	Comp($$anchor, { get id() {
		return value();
	} });
}
