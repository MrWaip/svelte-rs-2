import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, { random: Math.floor(Math.random() * 10) });
}
