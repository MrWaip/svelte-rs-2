import * as $ from "svelte/internal/server";
import Badge from "./Badge.svelte";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	function f(n) {
		return n;
	}
	Badge($$renderer, { text: `a ${$.stringify(f(x))} b` });
	$.bind_props($$props, { x });
}
