import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	var b;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => b = true]);
	$$renderer.async_block([$$promises[1]], ($$renderer) => {
		Child($$renderer, { b });
	});
}
