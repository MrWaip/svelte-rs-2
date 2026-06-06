import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let handler = (error) => console.error(error);
	Child($$anchor, { get handler() {
		return handler;
	} });
}
