import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	var b;
	var $$promises = $.run([() => Promise.resolve(), () => b = true]);
	{
		$.async($$anchor, [$$promises[1]], void 0, ($$anchor) => {
			Child($$anchor, { get b() {
				return b;
			} });
		});
		$.next();
	}
}
