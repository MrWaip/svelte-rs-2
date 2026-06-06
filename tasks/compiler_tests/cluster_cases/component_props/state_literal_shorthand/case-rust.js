import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let y = "y1";
	Child($$anchor, { y });
}
