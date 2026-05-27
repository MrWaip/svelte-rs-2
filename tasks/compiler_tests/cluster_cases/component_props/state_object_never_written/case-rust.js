import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	Child($$anchor, { get items() {
		return items;
	} });
}
