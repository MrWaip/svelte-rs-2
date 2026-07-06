import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	const SIZE = 4;
	Child($$renderer, { max: String(SIZE) });
}
