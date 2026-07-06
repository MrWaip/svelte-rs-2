import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let handler;
	let cond = $.fallback($$props["cond"], false);
	function a() {}
	function b() {}
	$: handler = cond ? a : b;
	Child($$renderer, {});
	$.bind_props($$props, { cond });
}
