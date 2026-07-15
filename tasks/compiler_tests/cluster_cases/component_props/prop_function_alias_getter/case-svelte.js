import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function close() {}
	const typedClose = close;
	Child($$anchor, { get handler() {
		return typedClose;
	} });
}
