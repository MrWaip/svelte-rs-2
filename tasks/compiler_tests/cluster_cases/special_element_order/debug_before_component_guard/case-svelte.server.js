import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let count = 0;
	console.log({ count });
	debugger;
	Child($$renderer, { count });
}
