import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let count = 0;
	count = 1;
	let doubled = $.derived(() => count * 2);
	Child($$renderer, { doubled: doubled() });
}
