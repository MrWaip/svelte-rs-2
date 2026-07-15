import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let value = void 0;
	Child($$anchor, { value });
}
