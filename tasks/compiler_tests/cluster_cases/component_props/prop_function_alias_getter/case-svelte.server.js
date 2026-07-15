import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function close() {}
	const typedClose = close;
	Child($$renderer, { handler: typedClose });
}
