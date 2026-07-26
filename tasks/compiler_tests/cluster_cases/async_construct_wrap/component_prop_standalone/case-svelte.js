import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	function delay(value) {
		return Promise.resolve(value);
	}
	{
		$.async($$anchor, void 0, [() => delay($$props.x)], ($$anchor, $0) => {
			Child($$anchor, { get value() {
				return $.get($0);
			} });
		});
		$.next();
	}
}
