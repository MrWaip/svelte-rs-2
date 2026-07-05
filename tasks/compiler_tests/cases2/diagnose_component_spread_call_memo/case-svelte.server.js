import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	function build(v) {
		return { value: v };
	}
	Child($$renderer, $.spread_props([build(value)]));
	$.bind_props($$props, { value });
}
