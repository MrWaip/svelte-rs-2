import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Child($$anchor, {
		"0": 0,
		"ysc%%gibberish": 1
	});
}
