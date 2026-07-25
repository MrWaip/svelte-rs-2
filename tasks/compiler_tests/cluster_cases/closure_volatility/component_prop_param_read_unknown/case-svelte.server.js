import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, { args: {
		f: (x) => x,
		g: undefined
	} });
}
