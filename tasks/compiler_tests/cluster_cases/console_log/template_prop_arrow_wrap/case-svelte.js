import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Child($$anchor, { onSelect: ({ detail }) => console.log("selected", detail) });
}
