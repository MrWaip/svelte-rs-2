import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	Child($$renderer, { items });
}
