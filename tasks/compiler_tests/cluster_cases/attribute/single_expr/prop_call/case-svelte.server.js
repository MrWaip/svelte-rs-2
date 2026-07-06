import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	function fn(v) {
		return v + 1;
	}
	Comp($$renderer, { id: fn(value) });
	$.bind_props($$props, { value });
}
