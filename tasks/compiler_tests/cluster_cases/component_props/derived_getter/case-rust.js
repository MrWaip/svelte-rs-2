import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let count = $.state(0);
	$.set(count, 1);
	let doubled = $.derived(() => $.get(count) * 2);
	Child($$anchor, { get doubled() {
		return $.get(doubled);
	} });
}
