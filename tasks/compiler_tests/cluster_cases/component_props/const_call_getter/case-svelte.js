import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const deferred = Promise.withResolvers();
	Child($$anchor, { get deferred() {
		return deferred;
	} });
}
