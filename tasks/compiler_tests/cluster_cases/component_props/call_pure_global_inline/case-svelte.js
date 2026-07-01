import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	Child($$anchor, { random: Math.random().toFixed(2) });
	$.pop();
}
