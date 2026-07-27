import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, $.spread_props([{ f: (x) => x }]));
}
