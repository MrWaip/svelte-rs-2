import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let y = "y1";
	Child($$renderer, { y });
}
