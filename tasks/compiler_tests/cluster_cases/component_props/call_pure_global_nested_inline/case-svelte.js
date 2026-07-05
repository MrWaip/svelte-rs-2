import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Child($$anchor, { random: Math.floor(Math.random() * 10) });
}
