import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	const deferred = Promise.withResolvers();
	Child($$renderer, { deferred });
}
