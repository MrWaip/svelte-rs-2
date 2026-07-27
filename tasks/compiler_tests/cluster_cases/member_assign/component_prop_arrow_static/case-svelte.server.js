import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	Child($$renderer, { onChange: (v) => obj.x = src });
}
