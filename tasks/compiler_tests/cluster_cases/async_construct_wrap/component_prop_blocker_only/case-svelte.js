import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $.run([async () => loaded = await delay(1)]);
	{
		$.async($$anchor, [$$promises[0]], void 0, ($$anchor) => {
			Child($$anchor, { get value() {
				return loaded;
			} });
		});
		$.next();
	}
}
