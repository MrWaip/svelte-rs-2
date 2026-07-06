import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let value = "";
	Child($$renderer, { value });
}
