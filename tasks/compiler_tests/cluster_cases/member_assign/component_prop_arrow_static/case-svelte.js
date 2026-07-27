import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	Child($$anchor, { onChange: (v) => obj.x = src });
}
