import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, { onSelect: ({ detail }) => console.log("selected", detail) });
}
