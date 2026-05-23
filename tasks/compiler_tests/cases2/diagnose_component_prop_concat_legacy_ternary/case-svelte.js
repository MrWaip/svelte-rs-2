import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	let toggle = $.prop($$props, "toggle", 8, false);
	{
		let $0 = $.derived_safe_equal(() => toggle() ? "A" : "B");
		Comp($$anchor, { get description() {
			return `prefix ${$.get($0) ?? ""}`;
		} });
	}
}
