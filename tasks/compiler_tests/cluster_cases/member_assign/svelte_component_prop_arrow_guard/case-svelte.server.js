import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	let C = Child;
	if (C) {
		$$renderer.push("<!--[-->");
		C($$renderer, { onChange: (v) => obj.x = src });
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
