import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function compute() {
		return 1;
	}
	{
		let $0 = $.derived_safe_equal(compute);
		Child($$anchor, { get track() {
			return $.get($0);
		} });
	}
}
