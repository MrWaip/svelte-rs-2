import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const label = "hi";
	Child($$anchor, { label });
}
