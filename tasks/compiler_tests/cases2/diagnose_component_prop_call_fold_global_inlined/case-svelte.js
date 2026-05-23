import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const SIZE = 4;
	{
		let $0 = $.derived(() => String(SIZE));
		Child($$anchor, { get max() {
			return $.get($0);
		} });
	}
}
